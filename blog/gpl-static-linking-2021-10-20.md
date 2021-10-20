---
title: "The GPL, linking and you"
date: 2021-10-20
tags:
 - gpl
 - gplv3
 - fsf
 - dynamic linking
---

# The GPL, linking and you

From the FSFs [GPL FAQ](https://www.gnu.org/licenses/gpl-faq.html#GPLStaticVsDynamic):

> Does the GPL have different requirements for statically vs dynamically linked modules with a covered work? (#GPLStaticVsDynamic)
> No. Linking a GPL covered work statically or dynamically with other modules is making a combined work based on the GPL covered work. Thus, the terms and conditions of the GNU General Public License cover the whole combination. See also What legal issues come up if I use GPL-incompatible libraries with GPL software?

I open with this quote because it's relevant to the subject. This post will go on a discussion on some pretty boring stuff, but it's mostly to illustrate that the legal FAQ for the GPL should be taken with a grain of salt. To be clear, I am not your lawyer, or even a lawyer. I'm just a bored software engineering student who feels this is relevant to elaborate on in an essay.

## A short introduction to the GPL

The GPL stands for "General Public License" according to the FSF. It's a software license for software where the source is available, although with a few additional requirements that make proponents call it "Free software". Roughly speaking, the idea behind the GPL is that if I choose to make my project open source, anyone should be able to use my project and my code, but they should do so under the same rules where I published my code. In other words, the GPL is a "I do my part, now you should do yours" license.

It's considered to be one of the more popular licenses for hobbyist/non-professional programmers, while corporations have generally shied away from the GPL or have begrudgingly complied with it in the past (sometimes with the [legal arm of the law](https://en.wikipedia.org/wiki/Open_source_license_litigation)).

## What is dynamic linking

The next thing to understand is dynamic linking. Dynamic linking is a solution to a problem that it's opposite, static linking creates. Let's say that we have a program that requests something from the internet. Let's call it program A for simplicity. Program A bundles all of it's code in the executable, which means we can put it on any machine and it'll work without issue. This sounds good in theory, but now we introduce Program B. Program B also requests something from the internet, using the exact same code that program A does. It also stores all of it's code in the executable, which again means it can be put on any machine and it works.

In theory, this would not cause any problems, but in reality this means that Program A and Program B share a lot of data. Data that doesn't actually *need* to be shared between programs. On a source code level, we can solve this by just flat out making program A and program B use the same library for development - we just get the source code of the library, import it in our program and if we change something in the library, when we recompile program A and program B with an update, it will reflect this.

This still creates two small problems - the first is that Program A and Program B now both are bigger than they kinda need to be. The compiled library code is identical after all. The second is that if the library has to make a change, you **have** to recompile program A and program B. This sounds like a non-issue, but when you get to libraries that are used by hundreds of millions of programs (for example libopenssl) that often have to get security fixes, you can easily end up with security flaws and subsequently hacks.

To solve these problems, a tradeoff can be made - a program can, instead of being compiled statically (which Program A and Program B are), be compiled dynamically. When you compile something dynamically, the resulting executable doesn't obtain a complete copy of the library in question. Instead, the compiler will scan the source code for references *to* said library and the resulting executable will only have a list of routines that the executable expects to be dynamically loaded from a shared library.

This creates the advantage that the shared code for Program A and Program B is kept *outside* of both Program A and Program B, which means it can be maintained and updated without having to modify either program A or program B. Instead, a library file is compiled alongside the programs (the exact nature of which varies. On Linux, you can usually identify these by the `.so` extension. Windows prefers `.dll` as an extension and finally Apple prefers `.dylib`), and that library file can then be kept out of actual distributions. This makes both Program A and Program B smaller, but has the tradeoff that the computer on which you want to run these programs needs a way to find the library file. (This is usually handled by the operating system, with a tool called a Dynamic Linker, which is beyond the scope of this discussion.)

## What the FSF misrepresents

Okay so now we're back at the FSF. So as my quote initially shows, the FSF is of the opinion that if you dynamically link your source code with a GPL library, you are still required to release said source code. Similarly, the LGPL, which is typically advertised as "GPL but you can dynamically link" only allows you to dynamically link the source code as a GPL library *if* and *only* if you release your source code as a dynamically linked program. Furthermore, the FSF also claims that if you write a plugin for a program and that program is GPL, then the plugin is under the GPL as well.

However, when examining legal precedent (most notably [Galoob v. Nintendo](https://en.wikipedia.org/wiki/Lewis_Galoob_Toys,_Inc._v._Nintendo_of_America,_Inc.) about the Game Genie, but also recently [Google v. Oracle](https://en.wikipedia.org/wiki/Google_LLC_v._Oracle_America,_Inc.)), the claim about the GPL rests on pretty shaky ground. In Galoob v. Nintendo, the US Ninth Circuit found that if you want to claim that your product is a derivative work, you actually have to *use* content from the work you are deriving from. In other words, if you aren't using any code that derives from the original work, you are not actually in violation of the license. Similarly, in Google v. Oracle, a supreme court decision concluded that usage of a public API can consist of a valid fair use defense, which has to do with the plugin claim.

What this means for dynamic libraries is quite interesting. It is theoretically possible to develop say, an MIT program internally with a fully functional, proprietary library, but can choose to not release said proprietary library (which is valid under the terms of the MIT).

However, if said MIT program's proprietary library matches exactly with a publicly available *GPL* library, then you can technically substitute that dynamic library for the GPL library and not be found in violation of the GPL license.

In a similar sense, the "plugin" claim doesn't hold up either. Because a plugin is fundamentally a different component that may not necessarily rely on the original program, you can theoretically develop the component for another program that just *happens* to be compatible with the GPL tooling. (for instance, let's say I have a screenshot tool that can load plugins under GPL and I have another screenshot tool that can load the exact same plugins but is MIT, then I can write a proprietary plugin for the MIT tool that happens to be compatible with the GPL tool, without having to relicense my proprietary plugin to GPL).

But here is the real kicker: In both of these examples I've talked about MIT tools that for a given definition are implied to actually exist. In reality, these tools probably don't even have to exist. They can exist hypothetically. As long as you can't actually *prove* that the code has to exclusively rely on the GPL libraries (which you can't, you can easily have a local reimplementation of these tools, which is common for the FSF to ironically even do themselves), and similarly if you can't *prove* that the plugin was written for the program exclusively, you can't actually claim that dynamically linking this source code is required.

Sure, you may be required to actually release the library code (in the program example) or release the program code (in the plugin example), but you are technically not actually required to release your own code under the GPL if you do this, as court precedent in the US seems to have shown.

## In conclusion

Okay so where does this lead us to... well not much really. I set out to prove that the FSF doesn't fully understand their own license, and I feel like I did that. If you are considering actually *using* the GPL, I personally would urge you to use the GPLv2 without the updating clause because the GPLv3 is bad (topic for another day). If you are an FSF cultist and this post made you unreasonably angry... idk take anger classes?

Remember that if you're in doubt, it's often better to actually read the license text itself rather than someone else's interpretation of it, as well as looking into potentially relevant court cases surrounding it. People rely on the FSF a lot for guidance on the GPL, but they're by no means an unbiased news source. Of course, I am not either. If you actually want a solid opinion either way, keep in mind that I am not a lawyer, I don't have a legal degree and you should talk to your actual lawyer.

Peace and have a great day.