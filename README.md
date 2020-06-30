# Simple Static Site Generator Rust (sssgr)

## TODO:

explain how this thing works 

## Layout

```
/ssgr
    /src
    /target
    /out     (where the html output and other files go)
    /in      (where the input files go)
        /res
            /images
            /pdfs
            favicon.ico
        /posts
        style.css
```

## Markdown parsing

Types of lines to process:
- Text line, can include inline links or code snippets
- subheading, prefixed with ##
- image, prefixed with \![alt text]
- code block, started with "\```" ending with "\```"
