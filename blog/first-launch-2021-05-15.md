---
title: New website!
date: 2021-05-15
---

# New website

Sup humans! It's ya girl. Here up in your bits with a totally new site. Okay well, not entirely new. Okay kinda new but the backend isn't. Fuck this is gonna need an explanation huh.

## Problems

So if you didn't know yet, this specific domain (https://noirscape.dev/) has been empty for... quite some time now actually! Mostly just because I didn't know what to put on it; I mostly work on things in a modular sense, so if I put something on the root, I felt it wouldn't represent me as accurately. I always sorta knew that I wanted it to be a blog/news site of sorts related about things I enjoy/want to talk about, but there's nothing in the world that could get me to really enjoy using wordpress and I'm not too big on a lot of the static site generators out there. So for a while I'd been looking and at last, suprise I found something I enjoy.

## xesite

So maybe you've seen [christine.website](https://christine.website/). Maybe you haven't and just realized how similar those two sites look. Well, the truth of the matter is fairly simple: I decided to fork it because I think it looks amazing and I liked the idea of the backend being something in rust. Luckily for me, the backend is FOSS and under the zLib license. Because I'm ~~feeling exceptionally nice~~ not an arsebiscuit, the source of the fork will be public as well. However, in accordance with the license, no I didn't make the original, this is a fork and with several removed features.

I find the best way to learn new things is to experience them as I go along and often working in an existing codebase can help greatly improve your own skills since you get a lot of reference code.

Whilst I decided to take out a **lot** of features that are honestly complete overkill for my site (I have no use for prometheus which that site uses for visitation tracking for instance, I use [shynet](https://github.com/milesmcc/shynet) for cookieless visitor tracking instead. I CARE ABOUT YOUR PRIVACY PEOPLE, I JUST WANT TO KNOW I'M NOT SHOUTING IN THE VOID!), the core stuff is there. I hope to tweak and adjust things as needed for the future. For me this is all very exciting, even if to you, the reader this is very boring and uninteresting.

## New site

So yeah, here we are now. Please do hold tight, a lot of stuff is still under construction and we're still testing a bunch of things here and there but this should be fun!

### How deployment works

Okay so this is mostly just interesting for me, since I *need* to document this process somewhere:

1. When locally writing a blogpost, just use `deploy.bat` (on Windows) or `deploy.sh` (on Linux) to get a quick local version of the site to test it.
2. You *normally* don't have to rebuild the application; that part is only needed when you change the templates in a heavy way. Don't do it if it's not needed, deploying takes a bit of time.
3. Once it's done, push the repo.
4. Then, pull the repo on the VPS.
5. Run `deploy_prod.sh` on the VPS.
6. ???
7. Profit.

If the templates were changed, make sure to generate a `musl` build! This avoids conflicts with GCC. Assuming cargo has musl installed, just use the following command: `cargo build --release --target x86_64-unknown-linux-musl` and upload the resulting binary to the server before pulling and restarting the repo.
