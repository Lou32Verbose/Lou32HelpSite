---
title: Bookmarklet Essentials
slug: /browser-web/bookmarklets/essentials/
summary: Small JavaScript bookmarklets for inspecting and simplifying web pages.
topic: browser-web/bookmarklets
type: template
tags: [bookmarklets, javascript, browser]
aliases: [bookmarklet list, browser helpers]
platforms: [browser, javascript]
related:
  - /cli-tools/wget/recursive-download/
status: published
updated: 2026-03-21
---

## Use Case

Keep a short set of reusable browser bookmarklets for inspection, cleanup, and quick navigation.

## Template

```javascript
javascript:(function(){
  alert(document.title + "\n" + location.href);
})();
```

## Variables

- `document.title`: current page title
- `location.href`: current page URL
- `document.links`: collection of links on the page
- `document.images`: collection of images on the page

## Examples

Copy the current page URL:

```javascript
javascript:(function(){
  navigator.clipboard.writeText(location.href);
  alert("Copied URL");
})();
```

List all links on the page:

```javascript
javascript:(function(){
  var list = Array.from(document.links).map(function(link) { return link.href; }).join("\n");
  alert(list || "No links found");
})();
```

Reader mode — removes clutter for easier reading:

```javascript
javascript:(function(){
  document.body.style.fontFamily='Arial, sans-serif';
  document.body.style.fontSize='16px';
  document.body.style.lineHeight='1.6';
  document.body.style.maxWidth='800px';
  document.body.style.margin='0 auto';
  document.body.style.padding='20px';
})();
```

Count words on current page:

```javascript
javascript:alert('Word count: '+document.body.innerText.split(/\s+/).length);
```

Select all text on page:

```javascript
javascript:document.execCommand('selectAll');
```

Go up one directory in URL:

```javascript
javascript:location.href=location.href.replace(/\/[^\/]*$/,'/');
```

Strip tracking parameters from URL:

```javascript
javascript:location.href=location.origin+location.pathname;
```

Count images on page:

```javascript
javascript:alert('Images: '+document.images.length);
```

Show basic page statistics:

```javascript
javascript:alert('Title: '+document.title+'\nURL: '+location.href+'\nImages: '+document.images.length+'\nLinks: '+document.links.length);
```

Invert page colors:

```javascript
javascript:document.documentElement.style.filter=document.documentElement.style.filter?'':'invert(1) hue-rotate(180deg)';
```

Highlight page background:

```javascript
javascript:document.body.style.backgroundColor='yellow';
```

Increase page zoom:

```javascript
javascript:document.body.style.zoom=parseFloat(document.body.style.zoom||1)+0.1;
```

Make page editable:

```javascript
javascript:document.body.contentEditable='true';document.designMode='on';alert('Page is now editable!');
```

Print-friendly mode — strips colors for printing:

```javascript
javascript:(function(){
  var style=document.createElement('style');
  style.innerHTML='*{background:white!important;color:black!important;}';
  document.head.appendChild(style);
})();
```

View page source:

```javascript
javascript:window.open('view-source:'+location.href);
```

List all JavaScript files loaded on page:

```javascript
javascript:(function(){
  var scripts=document.scripts;
  var list='';
  for(var i=0;i<scripts.length;i++){if(scripts[i].src)list+=scripts[i].src+'\n';}
  alert(list||'No external scripts found');
})();
```

Email current page link:

```javascript
javascript:location.href='mailto:?subject='+encodeURIComponent(document.title)+'&body='+encodeURIComponent(location.href);
```

Search current site via Google:

```javascript
javascript:window.open('https://www.google.com/search?q=site:'+location.hostname);
```

View page on Wayback Machine:

```javascript
javascript:window.open('https://web.archive.org/web/*/'+location.href);
```

Open speed test:

```javascript
javascript:window.open('https://fast.com');
```

Open browser Find dialog:

```javascript
javascript:window.find();
```

## Related

- [`Wget Recursive Download Reference`](/cli-tools/wget/recursive-download/)
