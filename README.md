# Keep To Markdown

A tool for migrating from [Google Keep](https://keep.google.com/) to Markdown. This was a quick project to help migrate to [Obsidian](https://obsidian.md/). It makes a couple different choices from the [built in importer](https://help.obsidian.md/import/google-keep):
- Puts the output in one file
- Includes the timestamp the note was created in the output

There may be edge cases or missing features - my goal for this project was just to migrate my own notes.

Here's how to use it:
- Download your Keep data from takeout.google.com (note: if this output contains data from anything other than keep, you may get errors or extra output)
- `cargo run -- path/to/takeout.zip > output.md`
