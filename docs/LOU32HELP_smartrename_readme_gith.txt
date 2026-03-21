## Input

### Search for

The text or regular expression to match in the item name
---
### Replace with

The text to replace the instance(s) in the item name matched by the Search text
---
## Options

### Use Regular Expressions

If checked, the Search field will be interpreted as a regular expression. The Replace field can also contain regex variables (see examples below).  If not checked, the Search field will be used as a text to be replaced with the text in the Replace field.
---
### Case Sensitive

If checked, the text specified in the Search field will only match text in the items if the text is the same case.  By default we match case insensitive.
---
### Match All Occurrences

If checked, all matches of the text in Search field will be replaced with the Replace text.  Otherwise, only the first instance of the Search for text in the item will be replaced (left to right).
---
### Exclude Files

Files will not be included in the operation.
---
### Exclude Folders

Folders will not be included in the operation.
---
### Exclude Subfolder Items

Items within folders will not be included in the operation.  By default, all subfolder items are included.
---
### Enumerate Items

Appends a numeric suffix to file names that were modified in the operation. 

Ex: foo.jpg -> foo (1).jpg
---
### Item Name Only

Only the file name portion (not the file extension) is modified by the operation.

Ex: txt.txt ->  NewName.txt
---
### Item Extension Only

Only the file extension portion (not the file name) is modified by the operation.

Ex: txt.txt -> txt.NewExtension
---
## Regular Expressions

For most use cases, a simple search and replace is sufficient.  Other users will need more control over.  That is where Regular Expressions come in.  Regular Expressions define a search pattern for text.  Regular expressions can be used to search, edit and manipulate text. The pattern defined by the regular expression may match one or several times or not at all for a given string.  SmartRename uses the ECMAScript grammar, which is common amongst modern programming languages.

To enable regular expressions, check the "Use Regular Expressions" checkbox. 

**Note:** You will likely want to check "Match All Occurrences" while using regular expressions.

### Examples

Simple matching examples:

| Search for     | Description                                           |
| -------------- | ------------- |
| .*             | Match all the text in the name                        |
| ^foo           | Match text that begins with "foo"                     |
| bar$           | Match text that ends with "bar"                       |
| ^foo.\*bar$     | Match text that begins with "foo" and ends with "bar" |
| .+?(?=bar)     | Match everything up to "bar"                          |
| foo[\s\S]\*bar | Match everything between "foo" and "bar"              |

Matching and variable examples:
**Note:** For using the variables, you do need "Match All Occurrences" enabled

| Search for | Replace With  | Description                                |
| ---------- | ------------- |--------------------------------------------|
| (.\*).png  | foo\_$1.png   | Prepends "foo\_" to the existing file name |
| (.\*).png  | $1\_foo.png   | Appends "\_foo" to the existing file name  |
| (.\*)      | $1.txt        | Appends ".txt" extension to existing file name |
| (^\w+\.$)\|(^\w+$) | $2.txt | Appends ".txt" extension to existing file name only if it does not have an extension |

### External Help

There are great examples/cheat sheets available online to help you

[Regex tutorial — A quick cheatsheet by examples](https://medium.com/factory-mind/regex-tutorial-a-simple-cheatsheet-by-examples-649dc1c3f285)

[ECMAScript Regular Expressions Tutorial](https://o7planning.org/en/12219/ecmascript-regular-expressions-tutorial)
