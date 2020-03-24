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

## Things that need to be written: 

 - something to generate the skelton of each page
 - something to generate the "special case" pages (index, all projects)
 - markdown parser (convert `.md` files to `.html` files)

## Things that need to be thought about:

 - how to decide what posts are going to be on the index page (and how they are ordered)
    * I think that chronological is the right answer, I'll need something similar for the all projects page as well



