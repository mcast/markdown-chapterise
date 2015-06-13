# markdown-chapterise
(vapourware // my first project in Rust)
## Split a [Markdown book](https://github.com/killercup/trpl-ebook) back into chapters, for nicer diffing.  

In [an issue on trpl-ebook](https://github.com/killercup/trpl-ebook/issues/15#issuecomment-108812768) I wrote:

> I'll have another poke around the old builds this evening, now I understand (for example) that 88c2bac is the last good-old-ruby build, and the book files are rebuilt several times (without changing the date-based name).

The PDFs I'm looking at / wanted to compare are from [trpl-2015-05-15.md, trpl-2015-05-24.md](https://github.com/killercup/trpl-ebook/commit/9e6b8c0), so I need to diff those .md's.

I propose to write a Markdown chapter splitter which takes a split depth (e.g. 2) and input `foo.md` to `foo.md.split/{}`, producing
* the YAML prologue as `prologue.yaml`
* `1-Introduction.md`; except when there's no content before sec 1.1
* `1.1-Contributing.md`
* `1.2-A_brief_introduction_to_Rust.md`
...

There is a risk of Github's diff viewer not working when section numbers change, but this could be fixed by rename commits before the diff commit.

Automating a viewable diff generator would be more work.

"I propose to write" kind of means...  I should write it in Rust I guess.  Hmm.

Matthew >= JAPH
