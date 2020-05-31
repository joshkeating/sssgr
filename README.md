# Simple Site Generator Rust (ssgr)

## Layout

```
/ssgr
    /src
    /target
    /out     (where the html output is going to go)
    /in      (where the input files go)
        /res
            /images
            /pdfs
            favicon.ico
        /posts
        style.css
```

## Things that need to work: 

 - something to generate the skelton of each page
> have the raw html for the page skelton 
 - something to generate the "special case" pages (index, all projects)
 - markdown parser (convert `.md` files to `.html` files)

## Things that need to be thought about:

 - how to decide what posts are going to be on the index page (and how they are ordered)
    * I think that chronological is the right answer, I'll need something similar for the all projects page as well


functions that are apparent:
    - generate page skeleton
    - generate card
    - inject card breakup
    - inject html from md
    - 


## Sample post metadata section

{{
document_link: ''
document_title: ''
date: ''
summary: ''
}}


would be cool to do a postproccessig pass with html5ever 

got the parsed tree of the skelton, need to inject the posts and see more html

write parser for markdown
