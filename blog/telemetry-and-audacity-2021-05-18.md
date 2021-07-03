---
title: Telemetry in FOSS & Audacity
date: 2021-05-18
---

Over the years, the attitude towards FOSS seems to have made... a turn.  Whereas originally, FOSS was the realm of the lunatic, the weirdo and that guy who kinda smelled like old funky cheese, nowadays many people end up using Free Software. As we see this grow, at times we see friction pop up between those who were there in the past and those who are in the community now, like for instance when RMS got ousted from the FSF (although he later slinked his way back in) for holding abhorrent views on non-programming related matters.

On an audience end, more and more people have started to use FOSS. I remember as a kid when the teacher in high school suddenly stopped using windows media player, the app I was thought to use as a kid to watch movies, listen to music and the like, and used this weird program called VLC. It was weird, it had this red road pylon on it, but eventually I got curious and since the high school I was on made the same programs available to students as they did for teachers, I quickly learned that VLC was in fact, just a movie player. I didn't learn it was FOSS until two years later.

That said, the discussion I want to touch upon today has to do with how FOSS has typically handled telemetry, how this differs from the conventional standards and how one recent example in particular has highlighted the difference in approach to how the original userbase approached telemetry with concern, while your average user is... frankly not that likely to really give a shit.

## Corporate distrust in FOSS

Perhaps unsurprisingly, most of the FOSS-heavy communities have a large distrust of corporations baked into them. This can be traced back to what can best be described as the first large FOSS movement, the GNU Foundation. (No Stallman didn't invent the idea of FOSS, but he was certainly the one that started the first collective movement that wasn't just a couple of nice people at MIT using a nicely permissive license.) The GNU Foundation, as it's ~~cultists~~ members will often try to remind you of was founded after Stallman was unable to modify a printer driver during his work at CSAIL. This particular event according to him caused him to resign from CSAIL and start working on what would eventually become the GNU/FSF and the coreutils. (The reality of this is somewhat different than most adherents would want to let you know. Stallman did quit CSAIL, but the late Marvin Minsky basically instantly reappointed him as a visiting professor, which gave him permanent residence at MIT until he made inappropriate comments surrounding the Epstein scandal in 2019).

From this event, a large trend would eventually set in that being "corporate" is bad, one propagated by the GNU Foundation, who for ages held the stance that "Not invented here syndrome" is actually a really good thing to apply to FOSS.

Whilst yours truly would of course agree that corporate fuckery can often be found afoot, for instance in programs like [Emby](https://emby.media/) (which went closed source and the open source fork is [Jellyfin](https://jellyfin.org/)), or the notorious email exchange between Linus and a couple of Intel engineers on the Linux mailing list [about the quality of Intel's choices for fixing the Specter CPU bug that made headlines in 2018](http://lkml.iu.edu/hypermail/linux/kernel/1801.2/04628.html), often this anti-corporate zeal can take a turn for the... conspiratorial, leading to frictions in incidents that I will talk about now.

So, with that in mind, let's now talk about Telemetry

## Telemetry

For those of you who don't know, Telemetry is when a program collects information on how you use it and reports it to a server, usually owned by whomever wrote the program to begin with.

Some amount of telemetry is unavoidable: A webapp couldn't work if it didn't collect the user's sign-in information. Twitter wouldn't work if you couldn't send tweets. The more... questionable kind of telemetry comes in when we look at telemetry that isn't really needed for the core operation of a program.

For a long while, this was more or less a sort of accepted fact; legislation around the matter was scarce and often inconclusive, which meant that a lot of nasty corporations could get away with collecting large amounts of data from users. On the other hand, non-essential telemetry could give the developer an insight in what kind of things are used more often in the program they wrote. (The modern video games "live service" model for instance, almost exclusively relies on this concept to see what features players engage with and what features they don't, which allows developers to prioritize what to work on, instead of wasting hours on features nobody would use.)

So telemetry, whilst being possible to use for evil, can also be used at least for some good: you can make sure that your software feels good to use and that people won't hate every living second of using the features they really need to use.

But of course, there is always one weird duck in the park. The anti-corporate FOSS community. The lawless fields of telemetry meant that a lot of corporate abuse was afoot, which meant that in general, most of the dislike towards telemetry got rolled into the fairly standard package of "we hate and distrust anything corporate" that a lot of the FOSS crowd seems really willing to repeat.

This eventually all would legally come to a head in the, now notorious, GDPR legislation from the European Union in 2018. The GDPR is something most people remembered for one of these two things:

* A major win for privacy watchdogs, users who valued their anonymity a lot and in general for people who wanted an insight on what companies collect on you.
* A fucking mess of software updates, terms of service changes and email spam for every service you have had to make an account for, for as long as you used the internet that the privacy policy was changing being send to you over about 5 or 6 months as companies scurried to implement it. Also, suddenly a lot of the news sites your American friends were linking you were made inaccessible.

Most people don't remember the first one. They remember the second one. As a result, whilst the people that still care about the GDPR will praise it high and low for adding privacy protections for users, most people just remember that vague daze from the time period of the second one and the very regular annoyance that is probably the reason that they use a VPN nowadays to visit some sites like they are from Japan.

The FOSS crowd still cares about the GDPR. They see it as an absolute win, because for them, very little was affected, since most FOSS tooling just flat out never used telemetry to begin with, and in the minds of a lot of FOSS people, "this finally would showcase the world how all the evil corporations collect your data". I would know, I was at that point in time, someone who was very much in on the "FOSS crowd", which included a pretty hefty bend of anti-corporate (I will leave it to someone else to find the embarrassing screenshots of me calling Microsoft "Micro$oft" unironically, because I remember doing that).

This didn't happen.

### The apathy of privacy

Instead, most people eventually sort of became... apathetic to it. Yeah big corporations collect your data. **So what?** is what the prevailing mindset turned into. And this is something that holds true for a lot of privacy. Most people just flat out **don't care about privacy except in a few specific instances**. For a good example of this, look no further than John Oliver's excellent episode about [Government Surveillance](https://www.youtube.com/watch?v=XEVlyP4_11M). 

Most of the people that were interviewed when talking about the, at the time relatively recent, Snowden leaks honestly couldn't care less about what the government collected on them, until Oliver warns them "yeah but they could also see your dick pics". Suddenly people cared. And this is a good example of where FOSS communities and regular users tend to miss the mark when talking to each other. Most people really don't care what you collect on them, as long as it's not deeply personal (like your dick pics) or if you're planning to be fairly obvious in doing nefarious things with them (Facebook collapsed in on itself because of global distrust in it's advertising practices). Hide it in a terms of services and chances are that you don't have to bother with anything, outside of that one smug neckbeard who thinks he's totally owning your privacy systems [speaking from experience, at one point I was that smug neckbeard].

I have a discord bot called Kirigiri that's decently popular. It's in ~600 guilds at the time of writing this blogpost, so I can safely say it's decently popular. It stores quite a bit of information for moderation reasons and in general processes a lot of information. As per the GDPR, I have to offer the option to get that removed for people. Nobody thus far has taken me up on this offer. I'm very open, very clean, nobody has ever asked me anything. Still no requests. This should tell you everything about how serious the regular user takes their own privacy.

## Audacity

So, now we come to Audacity. Audacity is one of those programs where the audience that generally uses it really just... isn't a part of the FOSS crowd. Audacity's main audience instead is content creators on YouTube and composers, and the overlap tends to be... minimal. There's almost assuredly more composers and creators who don't know about FOSS that use Audacity than those that do is what I'm trying to say. Almost every YouTuber I've watched in the past 8 years I could be legally considered a person on the internet (and add another 5 if you add those where I was breaking COPPA), all use Audacity.

Now regular users, as stated before don't care about privacy that much unless you get in their personal space or are planning to be nefarious. This means, that when Audacity was originally bought by "Muse Group", the same group of people who own MuseScore, a really popular composing tool, the FOSS crowd was hedging their bets, because of previous corporate fuckery, while the "content producer" community was excited to see Audacity finally get professional support.

Then a week later, the FOSS crowd got their [worst fears proven right](https://github.com/audacity/audacity/pull/835). Audacity is adding telemetry! Oh no! And not even the non-invasive kind, no they're partnering with *Google* and *Yandex*! This cannot be! (Google especially is a boogeyman in FOSS communities, for much the same reason most regular computer users don't like them but then amplified by about a 1000). That PR has 1.2k replies, most negative. FOSS people are passionate and they're certainly numerous. They also are [really easy to whip up](https://news.ycombinator.com/item?id=27068400) and drive out to respond in droves. 

Off-site however, from what I can find, reactions were much more relaxed on the matter and there were a couple of the usual "could the people who don't even use my software normally stop complaining about things being added that I really don't think are a big deal" responses that you always find in these things. Given that GitHub naturally incorporates a selection bias in who responds (GitHub is very developer oriented, not many normal users have accounts on the site).

Muse Group [walked it back](https://github.com/audacity/audacity/discussions/889) after the backlash in a public statement, which saw a fairly positive response from most users in the thread, which soothed basically all but the loudest voices who still claimed that Audacity has been tainted in the eyes of the FOSS crowd. A corporation in their eyes tried to fuck with it and they own it, so they can't trust it anymore. Best to fork and move on. This is a prevailing FOSS mindset, and whilst not always unwarranted, it can certainly be excessive sometimes and I think it happened here.

So what happened here? Well, if you ask me, we saw these two worlds clash. Whilst the telemetry's point and purpose is fairly straightforward (collect data on program usage, report non-critical bugs so that they can be ironed out, recognize that most users don't report bugs, that kinda stuff), the fact that the word telemetry was thrown around setting people off and the choice for Google being used for collection resulted in what would otherwise be a fairly "meh" thing being blown up into something much bigger.

Telemetry on corporate applications is considered a given, telemetry on FOSS isn't. Content producers are used to it because their other tools do it (ie. Youtubers use tools like Adobe Premiere, which yeah collects usage data [for much the same reasons](https://www.adobe.com/privacy/policy.html#info-use)), the FOSS crowd isn't used to it because prevailing mindset in those communities is to not do it.

Perhaps it also shouldn't come as a surprise then that many FOSS tools tend to suffer from extreme amounts of bugs that just aren't being reported upon. Just something to contemplate about.

## Conclusion

As for the specifics of who is in the right here; honestly my stance is probably the most lukewarm take you can get: Google was a bad choice, this was a bad PR moment and this should have been implemented much better. Telemetry isn't inherently evil, it has good uses. It has plenty of bad ones too, but this seems like a case of a developer used to a fairly milquetoast response from adding a feature that helps their development getting a lot harsher pushback than what they expected due to making some poor decisions.

They expected one audience but then accidentally got another. Mistakes happen. Assume cockup before malice is a useful guiding principle to have here. Very few people see themselves as the bad guy, and from what I can tell, all parties here tried to act in the best faith they saw fit.

Finally, on where to go with this; honestly for the FOSS crowd, I'd consider reading up a bit on what most computer users actually care about as opposed to what you think they care about. This was not an issue to steel yourselves over and condemning Audacity to the pits of corporate "tainted" FOSS (alongside for instance VS Code) is a bit of a silly mark. But that's my biggest critique of the FOSS crowd in general: passionate but directionless and incompetent on an insane amount of things. 

Get your shit together. Maybe then you can actually be a serious competitor instead of one that is only serious because FOSS is a good corporate marketing blurb.

## Amendment in July 2021

Well, this article aged poorly. Seems Musescore is [now trying to go open core](https://musescore.org/en/node/319957). For brevity's sake, I'll leave this article as is, but I obviously no longer have the positive reception I had initially to musescore's takeover attempt.
