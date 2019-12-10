# Mockery.rs

Generate fake data based on a specification file. Multiple models can be specified in a file, and models may reference
properties on each other to create relational data.

## Installation

Currently Mockery.rs is distributed via crates.io. Binary distributions will be available in the future

- `cargo install mockery` (Tested against rustc 1.39.0)

## CLI Usage

```markdown
mockery.rs 0.2.0
Louis Capitanchik <contact@louiscap.co>
Generate spec based model data.

USAGE:
    mockery [FLAGS] [OPTIONS] <MODEL> [OUTPUT]

FLAGS:
    -h, --help       
            Prints help information

    -p, --pretty     
            Whether or not the output should be formatted for human consumption. Default: false

    -V, --version    
            Prints version information


OPTIONS:
    -n, --number <NUMBER>       
            The number of root models that should be generated

    -s, --spec <SPEC_PATH>      
            Sets the spec file to use. By default, mockery will look for a 'spec.json' file in CWD, and will error if it
            can not be found
    -t, --type <OUTPUT_TYPE>    
            Sets the output type. This value defaults to CSV for higher compatibility and throughput [possible values:
            csv, json]

ARGS:
    <MODEL>     
            Sets the model to generate. The model determines what files will be generated based on it's definition in
            the spec
    <OUTPUT>    
            Sets the output path. Must be a file path pointing to a folder that optionally exists; if it does not exist,
            it will be created. Files corresponding to the input model names will be created inside this folder
```

## Getting Started

Models are defined in a `specification` file that maps models names to property/type pairs. Currently only `JSON` is 
supported, but more formats will be supported in the future. This short guide will walk you through setting up a model
spec that can generate a complex set of relational data

### A Basic Model

The minimum boilerplate for the `spec.json` file is as follows:

```json
{
  "models": {
  
  }
}
```

This defines an empty set of models. Obviously, this means that nothing can be generated. So we'll add a simple `post` model that
contains `id` (a UUID), `title` (a sentence) and `contents` (a paragraph) properties to represent a post on a micro-blogging 
service like Mastodon. Model names are keys within the `models` object, and they map to a nested object defining the model
attributes:

```json
{
  "models": {
    "post": {
      "id": {
        "type": "RandomData",
        "value": "UUID4"
      },
      "title": {
        "type": "RandomData",
        "value": "Sentence"
      },
      "contents": {
        "type": "RandomData",
        "value": "Paragraph"
      }
    }
  }
}
```

There are quite a few new parts here; we have defined a `post` model and three simple attributes. Each attribute has a
`type` and `value` that determines how it is generated. Specifying `RandomData` for the `type` allows us to generate a
rich set of mock data.

We chose to generate a v4 UUID for our `id` property using the `UUID4` value, a single sentence of
lorem ipsum text for our `title` with the `Sentence` value, and a single paragraph of lorem ipsum text for our `contents`.

This is enough to get started, but there are a few different values for `type` that can come in handy, and a way to
specify options for complex types in the `value` position of a `RandomData` attribute.

### Using Complex Types

Sometimes we want a bit of control over the sorts of data that get generated. For example, we might want a value to be
anything within a range. We may also sometimes just want to include static, predetermined data in our models; Complex
types provide syntax to customise the output value. In the following example, we'll build upon the previous example by
generating a cover image of a certain size to show alongside our post:

```json
{
  "models": {
    "post": {
      "id": {
        "type": "RandomData",
        "value": "UUID4"
      },
      "title": {
        "type": "RandomData",
        "value": "Sentence"
      },
      "contents": {
        "type": "RandomData",
        "value": "Paragraph"
      },
      "cover_image": {
        "type": "RandomData",
        "value": {
          "LoremPicsum": {
            "width": 1920,
            "height": 1080
          }
        }
      }
    }
  }
}
```

By adding the `cover_image` attribute, we now have a lorem picsum URL that will render a 1080p image. In order to specify
a type with options, we need to nest the definition within braces due to the nature of the JSON syntax. This is subject
to change before `1.0.0` depending on the shape of an alternative method. A list of available `RandomData` types can be
found [here]()

### Generating Data

Now that you have your model definition, there are a few things you can do to actual generate the data. This section
will assume that you have saved either of the previous model definitions in a file called `spec.json` in your current
working directory.

If you run the command `mockery post output`, Mockery.rs will create a file called `output/post.csv` which contains a single
post. You can generate more than one post by specifying `-n <number>`; for example `mockery post output -n 1000` will generate
a `post.csv` file that contains 1000 mock posts.

There's something strange about the rows though...the data isn't always in the same order on each row!

Mockery.rs doesn't generate or store the data in an ordered manner internally, so you need to add another section to your
`spec.json` file if you want the CSV output to be ordered. This isn't factored in when generating a JSON output, because
ordering shouldn't matter for a JSON file. If you have a usecase for ordering the keys in JSON output, please open an issue
with more information.

Adding a `serialize` key to the root of your `spec.json` file allows you to specify the property ordering for each model.
Be careful though; if you specify an order for a given model, only the keys that you list will be present in the output.

```json
{
  "serialize": {
    "post": [
      "id",
      "title",
      "contents",
      "cover_image"
    ]
  },
  "models": {
    "post": {
      "...": "..."
    }
  }
}
```

Now, when the output generates a CSV file, the `post` model will have all attributes ordered as listed. This output method
is good for insertion into a relational database, as many RDBMS systems have some form of CSV import that is faster than
traditional insertion queries.

If you wanted to output formatted json, though, you might want to run `mockery post output -t json -n 1000 -p`; this
will create an `output/post.json` file that includes 1000 pretty-printed posts wrapped in an array. This output format
is quite easy to skim through by eye, and omitting the `-p` flag will output a concise JSON format that can easily be 
used as part of a mock API.
