---
title: Example of a blogpost.
date: 2021-05-15
---

# Example of a blogpost

The following is a simple template/explanation on how to make blogs in this site's system. Consider it a half documentation of sorts.

## Blogposts are md files

Every file in the `blog` folder that ends with `.md` is considered a blog file. Their filename determines their slug. For instane, the file `2021-05-15-foo.md` will be accessible under `/blog/2021-05-15-foo`.

Make sure that there's always at least one blogpost present, otherwise you get issues!

Formatting is very straightforward: You can just use markdown for writing your posts.

## Front matter

Posts in this system get special tweaks depending on what you put in the front matter. For blogposts, only two are required: `title` and `date`.

Below is the absolute *minimum* you need to have valid front matter.

```yml
---
title: Foo
date: 01-01-1970
---
```

Anything else is optional but can be used for additional elements. Below is a full list of other keys you may choose to add:

* `series`: If this key is present, the post will be considered part of a "series". All series [are listed here](/blog/series). Mention that the post is a part of a series will furthermore be located in on the series page. Very straightforward, I can't imagine anyone screwing this up.
* `tags`: Tags are meanwhile are used in the json feed for the most part, appear as default hashtags when clicking share links and so on and so forth. They're a YAML list.
* `image`: Used for galleries only. A link to an image to be shown in the gallery.
* `thumb`: Used again, for galleries only. A link to the thumbnail to be shown on the thumbnail page.
* `redirect_to`: Inserts a javascript redirect to another URL entirely (namely what you fill in them). Consider it useful for linking external publications and essays.
* `article_title`: If this key is present, this overrides the title display on the blogpost itself. `title` will still be used for the list.

Outside of that, anything goes really!