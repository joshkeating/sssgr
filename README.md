# Simple Static Site Generator Rust (sssgr)

Builds my website from a directory of markdown files. Each markdown file contains the content for a single page or "post". For each file of markdown, a `.html` page is generated.

This is mostly a product of me getting tired of manually formatting something I wrote for the site. The simple layout and design of the outputted site definitely makes this a little easier. 

## Process

The high-level process of the program is goes like something like this:
1. Generate the correct layout of the output directory structure (see below)
2. Copy any static files from the input directory over to the output directory (eg. `.css` and `favicon.ico`)
3. Parse the markdown for each file. This step accomplishes the following:
    - Pulling out the "hidden" post metadata that will be used to:
        + name the file
        + populate an info card that will be used to display the post in other areas of the site
        + sort posts based on publish date of the markdown
    - Incrementally build a dom representation of the structure of the markdown input
    - Check for correct `html` serialization for the generated dom
    - Write dom to html output file
4. Generate a couple of pages that are not based on markdown (eg. the `index.html`)

## Default File Layout

The program expects a particular input file structure and will generate a set output file structure. These structures are very similar with the exception that the input contains `.md` files and the output`.html` files.

### Input file Structure (In project context)

```
/ssgr
└───/src
└───/target
└───/out    
└───/in
│   └───/res
│   │   └───/images
│   │   └───/pdfs
│   │   │   favicon.ico
│   └───posts
│   │   │   post1.md
│   │   │   post2.md
│   │   │   ...
│   │   style.css
```

### Output file Structure (In project context)

```
/ssgr
└───/src
└───/target
└───/out
│   └───/res
│   │   └───/images
│   │   └───/pdfs
│   │   │   favicon.ico
│   post1.html
│   post2.html
│   ...
│   style.css
└───/in 
```

## Markdown parsing

The current list of markdown that is supported is limited, but allows for a functional page layout:

- Text line, can include inline links or code snippets
- Subheading, prefixed with ##
- Image, prefixed with \![alt text]
- Code block, started with "\```" ending with "\```"

### TODO
- More flexible i/o options (eg. provide custom input path at runtime)
- Wider markdown support
- Reduce standard dom tree building code redundancy
- Hook into AWS for auto push of updates