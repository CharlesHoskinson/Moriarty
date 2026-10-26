# DIZK: A Distributed Zero Knowledge Proof System
> Source: https://www.usenix.org/conference/usenixsecurity18/presentation/wu
> Collected: 2026-10-26
> Published: Unknown

DIZK: A Distributed Zero Knowledge Proof System | USENIX

[Back to USENIX](/)

* [Sign In](/user/login?destination=conference/usenixsecurity18/presentation/wu)
* [Conferences](/conferences)

[![](https://www.usenix.org/sites/default/files/styles/neat_conference_menu_logo/public/sec18_wordmark_stacked_450x186.png)](/conference/usenixsecurity18)

* [Attend](/conference/usenixsecurity18/registration-information) 

  + [Registration Information](/conference/usenixsecurity18/registration-information)
  + [Registration Discounts](/conference/usenixsecurity18/registration-discounts)

  + [Student and Diversity Grants](/conference/usenixsecurity18/students-and-grants)

  + [Venue, Hotel, and Travel](/conference/usenixsecurity18/venue-hotel-and-travel)
  + [Services](/conference/usenixsecurity18/services)
* [Program](/conference/usenixsecurity18/glance) 

  + [At a Glance](/conference/usenixsecurity18/glance)
  + [Technical Sessions](/conference/usenixsecurity18/technical-sessions)

  + [Activities](/conference/usenixsecurity18/activities)
  + [Birds-of-a-Feather Sessions](/conference/usenixsecurity18/bofs)
  + [Poster Session and Happy Hour](/conference/usenixsecurity18/poster-session)
* [Participate](/conference/usenixsecurity18/call-for-papers) 

  + [Call for Papers](/conference/usenixsecurity18/call-for-papers)
  + [Submission Policies and Instructions](/conference/usenixsecurity18/submission-policies-instructions)

  + [Instructions for Presenters](/conference/usenixsecurity18/instructions-presenters)
* [Sponsorship](/conference/usenixsecurity18#sponsorship)
* [About](/conference/usenixsecurity18#about) 

  + [Symposium Organizers](/conference/usenixsecurity18/symposium-organizers)
  + [Questions](/conference/usenixsecurity18#questions)
  + [Past Symposia](https://www.usenix.org/conferences/byname/108)

  + [Conference Policies](/conferences/policies-resources)
  + [Code of Conduct](/conferences/coc)

DIZK: A Distributed Zero Knowledge Proof System
===============================================

Howard Wu, Wenting Zheng, Alessandro Chiesa, Raluca Ada Popa, and Ion Stoica, *UC Berkeley*

Recently there has been much academic and industrial interest in practical implementations of zero knowledge proofs. These techniques allow a party to prove to another party that a given statement is true without revealing any additional information. In a Bitcoin-like system, this allows a payer to prove validity of a payment without disclosing the payment’s details. Unfortunately, the existing systems for generating such proofs are very expensive, especially in terms of memory overhead. Worse yet, these systems are “monolithic”, so they are limited by the memory resources of a single machine. This severely limits their practical applicability. We describe DIZK, a system that distributes the generation of a zero knowledge proof across machines in a compute cluster. Using a set of new techniques, we show that DIZK scales to computations of up to billions of log- ical gates (100× larger than prior art) at a cost of 10μs per gate (100× faster than prior art). We then use DIZK to study various security applications.

Open Access Media
-----------------

USENIX is committed to Open Access to the research presented at our events. Papers and proceedings are freely available to everyone once the event begins. Any video, audio, and/or slides that are posted after the event are also free and open to everyone. [Support USENIX](/annual-fund) and our commitment to Open Access.

BibTeX

@inproceedings {217484,  
author = {Howard Wu and Wenting Zheng and Alessandro Chiesa and Raluca Ada Popa and Ion Stoica},  
title = {{DIZK}: A Distributed Zero Knowledge Proof System},  
booktitle = {27th USENIX Security Symposium (USENIX Security 18)},  
year = {2018},  
isbn = {978-1-939133-04-5},  
address = {Baltimore, MD},  
pages = {675--692},  
url = {https://www.usenix.org/conference/usenixsecurity18/presentation/wu},  
publisher = {USENIX Association},  
month = aug  
}

[Download](/biblio/export/bibtex/217484)

![PDF icon](/core/modules/file/icons/application-pdf.png "application/pdf")[Wu PDF](https://www.usenix.org/system/files/conference/usenixsecurity18/sec18-wu.pdf "sec18-wu.pdf")

[View the slides](https://www.usenix.org/sites/default/files/conference/protected-files/security18_slides_wu_0.pdf)

Presentation Video
------------------

#### Presentation Audio

[MP3 Download](https://2459d6dc103cb5933875-c0245c5c937c5dedcca3f1764ecc9b2f.ssl.cf2.rackcdn.com/sec18/wu.mp3)

[Download Audio](https://2459d6dc103cb5933875-c0245c5c937c5dedcca3f1764ecc9b2f.ssl.cf2.rackcdn.com/sec18/wu.mp3)

* [Attend](/conference/usenixsecurity18/registration-information)
  + [Registration Information](/conference/usenixsecurity18/registration-information)
  + [Registration Discounts](/conference/usenixsecurity18/registration-discounts)
  + [Student and Diversity Grants](/conference/usenixsecurity18/students-and-grants)
  + [Venue, Hotel, and Travel](/conference/usenixsecurity18/venue-hotel-and-travel)
  + [Services](/conference/usenixsecurity18/services)
* [Program](/conference/usenixsecurity18/glance)
  + [At a Glance](/conference/usenixsecurity18/glance)
  + [Technical Sessions](/conference/usenixsecurity18/technical-sessions)
  + [Activities](/conference/usenixsecurity18/activities)
  + [Birds-of-a-Feather Sessions](/conference/usenixsecurity18/bofs)
  + [Poster Session and Happy Hour](/conference/usenixsecurity18/poster-session)
* [Participate](/conference/usenixsecurity18/call-for-papers)
  + [Call for Papers](/conference/usenixsecurity18/call-for-papers)
  + [Submission Policies and Instructions](/conference/usenixsecurity18/submission-policies-instructions)
  + [Instructions for Presenters](/conference/usenixsecurity18/instructions-presenters)
* [Sponsorship](/conference/usenixsecurity18#sponsorship)
* [About](/conference/usenixsecurity18#about)
  + [Symposium Organizers](/conference/usenixsecurity18/symposium-organizers)
  + [Questions](/conference/usenixsecurity18#questions)
  + [Past Symposia](https://www.usenix.org/conferences/byname/108)
  + [Conference Policies](/conferences/policies-resources)
  + [Code of Conduct](/conferences/coc)

[Twitter](https://twitter.com/usenixsecurity)[Facebook](https://www.facebook.com/USENIX-Security-Symposium-13544602263)[Google](https://plus.google.com/108588319090208187909/posts)[Youtube](https://www.youtube.com/user/USENIXAssociation)

* [Privacy Policy](/privacy-policy)
* [Contact Us](/contact)

© USENIX   
EIN 13-3055038
