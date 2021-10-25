---
title: Deprecating Kirigiri
date: 2021-10-25
---

# Deprecating Kirigiri

It is with a heavy heart that I hereby announce the formal cancellation of any and all development on the public instance of the Kirigiri discord bot.

The bot will be kept running for as long as I am able, but there is an ending in sight for most of the bot's functionality, with a permanent shutdown being eventually on the way.

## Why

The short of it boils down to "Discord is a terrible company which doesn't understand the usefulness bots can have". The long version will boil down to a multitude of reasons, which I will elaborate on below.

* First and foremost, [Discord is making message content a privileged intent](https://support.discord.com/hc/en-us/articles/4410940809111). I will not wash any words over this. This is a terrible policy. This is a policy that fundamentally betrays the amount of work, time and respect many people have put into using bots to make Discord a usable platform. Discord has explicitly stated that they will not consider basic command interpretations as a valid reason to access message contents, nor do they consider starboard to be a useful one, and while I *could* easily provide or mock up some sort of compelling feature, I frankly lack the energy to do so, nor do I think Discord would accept it.
* The second reason is a more complex one. I would suggest starting with [Danny's gist](https://gist.github.com/Rapptz/4a2f62751b9600a31a0d3c78100287f1). The short of it is that Discord has habitually treated bot developers as second rate customers, who at best get treated with mild contempt and at worst get treated with outright dismissal of concerns. Discord habitually messes with the API, habitually makes things more complex for developers, constantly attempts to reimplement features bots provide but in an objectively worse way and the worst of all is that developers get blamed for stuff that developers **aren't even doing**. I will elaborate on that in a bit.
* The final reason is related to the second; Discord.py is unmaintained, and while forks do exist (one of those I use personally), I frankly don't have the ability and/or time to migrate this bot over to a new library or a fork. The bot's codebase is near sysiphean in nature by now, consisting of several weird design decisions made from a weird mindest of "portability" I once had in mind for the bot, and as a result, I consider the bots codebase to be firmly in the "patch it to make it work" state.

## Discords habitual mistreatment of bot developers

Like a lot of early Discord bot developers, I started developing bots as a way to get into programming. I'd been learning it in university, sure, but I really wanted a fun thing to get my hands on. At the time, Discord had a much more lax policy on selfbots; the rule was basically that if nobody else could interact with your selfbot and if your selfbot wasn't putting massive stress on the API (spamming deletions that hit the rate limit for example), nobody gave a fuck.

That eventually changed. Discord suddenly took a harsh stance against selfbots. The ostensible reason boiled down to "people use them to spam servers", a reason that while true, was still a blow to the selfbot "community" (I had contributions to one of the larger selfbots out there). Unfortunate, but that's how things go. There is a small bit of irony in that Discord only ever considered an actual bot API after people were making userbots, but oh well.

Eventually I decided I wanted some functionality back; there were a bunch of cool things you could do with a selfbot that a regular moderation bot could also help you with. From that idea, numerous small and not so small feature bots were born; most of which lie archived on my Github profile as I've moved on from one idea to another.

Then, in 2018-2019, while suffering through an extended mental breakdown, I decided to try and prove to mostly myself that I was not a total wreck of a human being: I would make a moderation bot, one that could be used on multiple servers, one I would be proud of hosting and one that people would like using. That bot would end up being Kirigiri.

During most of this period, Discord has not done that much with bots. That... changed in 2020. At first the changes were at least interesting; we were getting official ability to implement "slash commands", a much desired feature from client modifications. Then we got the ability to add UIs to messages, also pretty cool. But then... things went south.

So I mentioned selfbots before. Well, because Discord banned casual selfbot usage, there was only really one group using them: member list scrapers and spammers. It takes little to no skill to notice that Discord has a *massive* spam problem. Numerous servers I'm in have outright filtered discussion of the word "nitro", just because spambots post fake links to hijack accounts. Member list scrapers meanwhile are a different issue; they're intended to profile server members and generate advertisement lists to then send them all sorts of random skeezy scams. **They are a big issue**.

So, obviously Discord's developers should make this their biggest priority for fixing this. Right? RIGHT? Well, they kinda did. The changes Discord announced at first were not related to curbing spam; Discord would announce that they would limit the amount of gateway events send to a bot account to only those that the bot would request. In theory, this is awesome -> bots can do a lot of optimization by dismissing certain kinds of events. Then... Discord announced privileged intents.

Unlike normal intents, you need to enable Privileged Intents in the apps menu, and if you are above 100 guilds (Discord calls servers guilds internally) with your bot, you need to do an additional application process.

On the surface, the first two Privileged Intents made a bit of sense: you couldn't query the member list and you couldn't query the users current status. Since both of these were obvious scraping targets this would make sense right? Well... this is where the conversation gets stupid. You see, these scrapers and spammers **aren't using the bot gateway**, because hitting that as a user is a ticket to "your ass is banned"-ville. So, for the first time, bot developers get blamed for the actions that the spambots are doing.

But hey, Discord was going to be nice about it; prove you're a human being, prove that you need it (and the criteria for this were really lax; I even screwed up my application and they helped me correct it), and you could get the Privileged Intents. Given I already had nitro at the time, I opted to just do it.

Now we fast forward to a few months ago. This was when Discord announced that [Message contents would now become a privileged intent in April 2022](https://support.discord.com/hc/en-us/articles/4410940809111), with the stated reason now being "we care about user privacy" (which was the same reason for the first two Intents; aka they want to do this to stop spambots). This pretty much kills all the old bots. If you aren't moving over to slashcommands, you're basically screwed.

The thing is... slash commands are kinda terrible? There's many bugs in them, they aren't remotely as capable of content parsing as we can do with just *reading message content* and in general they are a feature downgrade. They also have weird ideas about message deference and demand you respond with something no matter what. I considered looking into them, but all I can conclude is that if Discord wants to make this a thing, my participation in that will not be happening.

But it gets *dumber*. You see, Discord's policies are infamously hypocritical. To put it in a way everyone can follow, it's easy to point to that one prolific case where a Trust & Safety member [allowed drawn furry child porn](https://www.polygon.com/2019/1/30/18203692/discord-nsfw-policy-furry-cub), but banned lolicon/shotacon (similar content but with anime characters). They eventually [changed that stance](https://www.polygon.com/2019/2/13/18223726/discord-policy-change-nsfw-cub-furry). Meanwhile it takes me barely *any* effort to find a public Discord server index that openly uses "cub" as an available tag. In a similar sense, it takes me just as little effort to find Nazi content, far right content, and so many other things that openly go against Discord Terms of Service.

And this just as much applies to bots too. I have numerous anecdotes available where people were running data farms on discord users (userbots), but when reported to Discord, the accounts just keep existing. Similarly, on the handful of cases where people were actually found to be running data farms with these intents, Discord pretty much ate the onion hook line and sinker, and approved them for Privileged Intents. Many of those still *are* running data farms. To put it simply; this approval process is **fucking stupid and doesn't work**.

So, with all that. I am just going to quote Mason, Discord API developers [stance](https://gist.github.com/Rapptz/4a2f62751b9600a31a0d3c78100287f1) on the forced move to slash commands:

> Uh, popular libraries. So, we obviously do not have control over third party libraries. They are third party. Um, we try to work with them very closely to keep them informed of changes, uh and to answer their questions about implementation and API and consistencies and "Hey does this API suck or not". Uh, Slash Commands have been in an open developer beta since December of last year. Changes are not happening until an additional 9 months from now. Which will make it 1 year and 4 months slash 16 months if I know how to do math correctly since Slash Commands were first released. Um many libraries do have support. Some have unofficial forks. I will also say that for those intrepid developers they are pretty well usable without libraries but I know that that is a much sort of bigger task to undertake. Um but if you haven't checked out using Slash Commands over HTTP and outgoing webhooks, it's pretty dope and really easily scalable.

Mason here encourages you to rewrite your Discord bot because it's "easy" and "dope". It really isn't. Kirigiri's codebase and featurescope is so unbelievably huge that reimplementing any of it to a new library is borderline sysiphean and unworkable. Porting it over to slashcommands is similarly, just not a project I have the energy for as a hobby. If this is Discord's official stance, then **I am fucking out**. If you still have a Discord bot, please, do read this. This is the value that Discord places on you. Decide if you want to be that kinda pawn.

I am not that pawn.

## How

Well, okay, that rant is over. Now let's actually talk about how this bot will be deprecated. This is the formal deprecation timeline, based on my own considerations (datetime is DD/MM/YYYY):

* 25/10/2021: Initial deprecation blogpost gets announced. Nothing will change for now.
* 12/01/2022: Script will be ran on the bot to inform all guilds of deprecation, with a priority strategy -> log channels are prioritized, if not available, DM guild owners. Should all those fail, I consider my bot to have made the maximum attempt to contact every user of the deprecation.
* 01/04/2022-30/04/2022: Discord deprecates message content intents for non-privileged users. Whenever this happens, Kirigiri will be formally disabled. I am unsure of the actual date, but when it happens, I will recontact every guild again to notify them.
* 01/04/2022-30/04/2022 (after previous): I will make it possible for guild owners to retrieve their guild specific JSON files, containing material such as warns, role attachments and other assigned content, in the hopes that they can use this to migrate their stuff over to a new bot.
* 01/07/2022: All guild specific data will be purged from my servers at this point in time, leaving no data retrieval possible.

## What's next

Even though I am deprecating the public bot, development on Kirigiri's codebase is not cancelled. Instead, I will be working to a more single-server focused strategy, which allows for message intents. This should allow for easier and more flexible development, while still maintaining what made the bot fun. If you are a user of the public instance, I will gladly be there to help you set up your own instance with the data I can provide you.

That's all.