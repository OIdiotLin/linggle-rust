# linggle-rust

<div align="center"><img src="https://linggle.com/static/img/linggle-logo.png" /></div>

[![Build Status](https://travis-ci.org/OIdiotLin/linggle-rust.svg?branch=master)](https://travis-ci.org/OIdiotLin/linggle-rust)


A CLI tool of [Linggle(ZH](https://linggle.com)/[EN)](https://linggle.com/en) implemented in rustlang.

## Installation

### Arch Linux

Installing from [aur](https://aur.archlinux.org/packages/linggle-git/) is recommended.

You could use [yay](https://github.com/Jguer/yay) to install it.

```bash
yay linggle-git
```

### Other Systems

1. clone this repo 
2. compile it: `cargo build --release`
3. `cp target/release/linggle /usr/bin`

## Usage

### Writing Ahead

Typing a few words and then a couple of underscores `_` allows you to find recurring phrases following the words.

```bash
>>> linggle "present a method _"
 1. 12227|48.79% 	present a method for
 2. 5726|22.85% 	present a method to
 3. 2983|11.90% 	present a method of
 4. 1986|7.92% 	present a method that
 5. 758|3.02% 	present a method which
 6. 536|2.14% 	present a method based
 7. 198|0.79% 	present a method by
 8. 134|0.53% 	present a method and
 9. 107|0.43% 	present a method in
10. 105|0.42% 	present a method using
11. 92|0.37% 	present a method called
12. 72|0.29% 	present a method where
13. 51|0.20% 	present a method whereby
14. 44|0.18% 	present a method allowing
15. 41|0.16% 	present a method how
```

### Checking Whether a Word Is Needed

A question mark `?` followed by a word (e.g., to) allows you to check whether the word is needed in a phrase.

```bash
>>> linggle "go ?to ask your mom"
 1. 189|100.00% 	go ask your mom
```

### Deciding on Alternative Phrases

Don't know which word is better? Use `/` to check it out. Phrases with a high frequency count are usually preferred.

```bash
>>> linggle "not in the/a position to"
 1. 254358|94.59% 	not in a position to
 2. 14534|5.41% 	not in the position to
```

### Finding Collocations

Linggle's unique and very best feature is that keywords can be combined with any part of speech to create a search query for finding collocations. For example, Linggle `v. death penalty` to find verb collocates of `death penalty`. The results are on par with what is available in a collocation dictionary.

```bash
>>> linggle "v. death penalty"
 1. 4521|12.41% 	face death penalty
 2. 2405|6.60% 	seek death penalty
 3. 2395|6.58% 	faces death penalty
 4. 2302|6.32% 	replacing death penalty
 5. 2085|5.73% 	get death penalty
 6. 1581|4.34% 	abolish death penalty
 7. 1570|4.31% 	state death penalty
 8. 1066|2.93% 	facing death penalty
 9. 1009|2.77% 	limiting death penalty
10. 947|2.60% 	seeks death penalty
...
```

### Precise Collocation Queries

Linggle uses a simple strategy to handle queries that involve parts of speech. Yet, POS can be ambiguous at times, leading to less than perfect results when it comes to POS queries. For example, the query **v. difficulty** returns **learning difficulty** as the top result, while **play n.** brings back **play station**! In fact, both should be classified as NN. 

There are other problems with using a simple search query like **v. difficulty**: missing verbal phrase (run into) and a lack of information related to articles in the retrieved phrases. For this, it would be better to use a slightly more complex query, e.g., **pron. v. ?prep. ?det. difficulty**. Here **pron.** is added so that **v.** can match a real verb (instead of seemingly correct verbs like **learning**). The optional POS wildcard **?prep.** after **v.** allows phrasal verbs(e.g., **run into**) to be found. The **?det.** before difficulty allows us to find the use of articles.

```bash
>>> linggle "to v. ?prep. ?det. difficulty/difficulties"
 1. 42201|22.11% 	to have difficulty
 2. 13761|7.21% 	to overcome the difficulties
 3. 12908|6.76% 	to overcome difficulties
 4. 12553|6.58% 	to have difficulties
 5. 6515|3.41% 	to experience difficulties
 6. 6049|3.17% 	to continue despite difficulties
 7. 5031|2.64% 	to experience difficulty
 8. 4682|2.45% 	to avoid difficulties
 9. 4640|2.43% 	to face difficulties
10. 4413|2.31% 	to resolve difficulties
11. 3971|2.08% 	to address the difficulties
12. 3393|1.78% 	to overcome the difficulty
13. 3296|1.73% 	to cause difficulties
14. 3182|1.67% 	to understand the difficulties
15. 3123|1.64% 	to increase the difficulty

```