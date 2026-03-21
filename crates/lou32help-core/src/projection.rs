use crate::config::{Lou32HelpConfig, TopicConfig};
use crate::document::Document;
use crate::library::TopicNode;
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub fn visible_documents(docs: &[Document], include_drafts: bool) -> Vec<&Document> {
    docs.iter()
        .filter(|doc| include_drafts || doc.is_published())
        .collect()
}

pub fn slug_index<'a>(docs: &[&'a Document]) -> HashMap<String, &'a Document> {
    docs.iter()
        .map(|doc| (doc.metadata.slug.clone(), *doc))
        .collect()
}

pub fn alias_index<'a>(docs: &[&'a Document]) -> HashMap<String, &'a Document> {
    let mut aliases = HashMap::new();
    for doc in docs {
        for alias in &doc.metadata.aliases {
            aliases.insert(alias.clone(), *doc);
        }
    }
    aliases
}

pub fn explicit_related<'a>(
    slug_index: &HashMap<String, &'a Document>,
    doc: &'a Document,
) -> Vec<&'a Document> {
    doc.metadata
        .related
        .iter()
        .filter_map(|slug| slug_index.get(slug).copied())
        .collect()
}

pub fn backlinks<'a>(docs: &[&'a Document], doc: &'a Document) -> Vec<&'a Document> {
    docs.iter()
        .copied()
        .filter(|candidate| {
            candidate.metadata.slug != doc.metadata.slug
                && candidate
                    .metadata
                    .related
                    .iter()
                    .any(|related| related == &doc.metadata.slug)
        })
        .collect()
}

pub fn computed_related<'a>(
    docs: &[&'a Document],
    doc: &'a Document,
    related_limit: usize,
) -> Vec<&'a Document> {
    let explicit = doc.metadata.related.iter().collect::<BTreeSet<_>>();
    let mut candidates = docs
        .iter()
        .copied()
        .filter(|candidate| candidate.metadata.slug != doc.metadata.slug)
        .filter(|candidate| !explicit.contains(&candidate.metadata.slug))
        .map(|candidate| {
            let same_topic = if candidate.metadata.topic == doc.metadata.topic {
                40
            } else if candidate
                .metadata
                .topic
                .starts_with(&format!("{}/", doc.top_level_topic()))
                || doc
                    .metadata
                    .topic
                    .starts_with(&format!("{}/", candidate.top_level_topic()))
            {
                20
            } else {
                0
            };

            let shared_tags = candidate
                .metadata
                .tags
                .iter()
                .filter(|tag| doc.metadata.tags.contains(tag))
                .count() as i32
                * 15;

            let shared_platforms = candidate
                .metadata
                .platforms
                .iter()
                .filter(|platform| doc.metadata.platforms.contains(platform))
                .count() as i32
                * 5;

            (candidate, same_topic + shared_tags + shared_platforms)
        })
        .filter(|(_, score)| *score > 0)
        .collect::<Vec<_>>();

    candidates.sort_by(|left, right| {
        right
            .1
            .cmp(&left.1)
            .then_with(|| left.0.metadata.title.cmp(&right.0.metadata.title))
    });
    candidates.truncate(related_limit);
    candidates.into_iter().map(|(doc, _)| doc).collect()
}

pub fn topic_nodes(config: &Lou32HelpConfig, docs: &[&Document]) -> BTreeMap<String, TopicNode> {
    let mut nodes = BTreeMap::<String, TopicNode>::new();

    for topic in &config.topics {
        nodes.insert(
            topic.key.clone(),
            TopicNode {
                path: topic.key.clone(),
                title: topic.title.clone(),
                description: Some(topic.description.clone()),
                parent: None,
                children: Vec::new(),
                documents: Vec::new(),
                order: topic.order,
            },
        );
    }

    for doc in docs {
        let segments = doc.metadata.topic.split('/').collect::<Vec<_>>();
        let mut current = String::new();

        for (index, segment) in segments.iter().enumerate() {
            if !current.is_empty() {
                current.push('/');
            }
            current.push_str(segment);

            let parent = if index == 0 {
                None
            } else {
                Some(segments[..index].join("/"))
            };

            let title = if index == 0 {
                config
                    .topic_by_key(segment)
                    .map(|topic| topic.title.clone())
                    .unwrap_or_else(|| config.topic_title(&current))
            } else {
                config.topic_title(&current)
            };

            let description = if index == 0 {
                config
                    .topic_by_key(segment)
                    .map(|topic| topic.description.clone())
            } else {
                None
            };

            let order = if index == 0 {
                config
                    .topic_by_key(segment)
                    .map(|topic| topic.order)
                    .unwrap_or(usize::MAX)
            } else {
                usize::MAX
            };

            nodes.entry(current.clone()).or_insert(TopicNode {
                path: current.clone(),
                title,
                description,
                parent,
                children: Vec::new(),
                documents: Vec::new(),
                order,
            });
        }

        if let Some(node) = nodes.get_mut(&doc.metadata.topic) {
            node.documents.push(doc.metadata.slug.clone());
        }
    }

    let child_pairs = nodes
        .values()
        .filter_map(|node| {
            node.parent
                .as_ref()
                .map(|parent| (parent.clone(), node.path.clone()))
        })
        .collect::<Vec<_>>();

    for (parent, child) in child_pairs {
        if let Some(node) = nodes.get_mut(&parent) {
            node.children.push(child);
        }
    }

    let order_lookup = nodes
        .iter()
        .map(|(path, node)| (path.clone(), node.order))
        .collect::<HashMap<_, _>>();

    for node in nodes.values_mut() {
        node.children.sort_by(|left, right| {
            let left_order = order_lookup.get(left).copied().unwrap_or(usize::MAX);
            let right_order = order_lookup.get(right).copied().unwrap_or(usize::MAX);
            left_order.cmp(&right_order).then_with(|| left.cmp(right))
        });
        node.documents.sort();
    }

    nodes
}

pub fn topic_documents<'a>(docs: &[&'a Document]) -> HashMap<String, Vec<&'a Document>> {
    let mut topics = HashMap::<String, Vec<&'a Document>>::new();

    for doc in docs {
        let mut current = String::new();
        for segment in doc.metadata.topic.split('/') {
            if !current.is_empty() {
                current.push('/');
            }
            current.push_str(segment);
            topics.entry(current.clone()).or_default().push(*doc);
        }
    }

    for entries in topics.values_mut() {
        entries.sort_by(|left, right| left.metadata.slug.cmp(&right.metadata.slug));
    }

    topics
}

pub fn tag_index<'a>(docs: &[&'a Document]) -> BTreeMap<String, Vec<&'a Document>> {
    let mut tags = BTreeMap::<String, Vec<&'a Document>>::new();
    for doc in docs {
        for tag in &doc.metadata.tags {
            tags.entry(tag.clone()).or_default().push(*doc);
        }
    }
    for entries in tags.values_mut() {
        entries.sort_by(|left, right| left.metadata.slug.cmp(&right.metadata.slug));
    }
    tags
}

pub fn recent_documents(mut docs: Vec<&Document>) -> Vec<&Document> {
    docs.sort_by(|left, right| {
        right
            .metadata
            .updated
            .cmp(&left.metadata.updated)
            .then_with(|| left.metadata.title.cmp(&right.metadata.title))
    });
    docs
}

pub fn top_level_topics_with_counts<'a>(
    config: &'a Lou32HelpConfig,
    docs: &[&Document],
) -> Vec<(&'a TopicConfig, usize)> {
    let mut counts = HashMap::<&str, usize>::new();
    for doc in docs {
        *counts.entry(doc.top_level_topic()).or_default() += 1;
    }

    let mut topics = config
        .topics
        .iter()
        .map(|topic| (topic, *counts.get(topic.key.as_str()).unwrap_or(&0)))
        .collect::<Vec<_>>();
    topics.sort_by(|left, right| left.0.order.cmp(&right.0.order));
    topics
}

pub fn platforms(docs: &[&Document]) -> Vec<String> {
    let mut platforms = docs
        .iter()
        .flat_map(|doc| doc.metadata.platforms.iter().cloned())
        .collect::<Vec<_>>();
    platforms.sort();
    platforms.dedup();
    platforms
}
