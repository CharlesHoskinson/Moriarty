[![W3C](https://www.w3.org/Icons/w3c_home)](https://www.w3.org/)

PROV-O: The PROV Ontology
=========================

W3C Recommendation 30 April 2013
--------------------------------

This version:
:   [http://www.w3.org/TR/2013/REC-prov-o-20130430/](https://www.w3.org/TR/2013/REC-prov-o-20130430/)

Latest published version:
:   [http://www.w3.org/TR/prov-o/](https://www.w3.org/TR/prov-o/)

Implementation report:
:   [http://www.w3.org/TR/2013/NOTE-prov-implementations-20130430/](https://www.w3.org/TR/2013/NOTE-prov-implementations-20130430/)

Previous version:
:   [http://www.w3.org/TR/2013/PR-prov-o-20130312/](https://www.w3.org/TR/2013/PR-prov-o-20130312/)

Editors:
:   [Timothy Lebo](http://tw.rpi.edu/instances/TimLebo), Rensselaer Polytechnic Institute, USA
:   [Satya Sahoo](http://cci.case.edu/cci/index.php/Satya_Sahoo), Case Western Reserve University, USA
:   [Deborah McGuinness](http://tw.rpi.edu/instances/Deborah_L_McGuinness), Rensselaer Polytechnic Institute, USA

Contributors:
:   (In alphabetical order)
:   [Khalid Belhajjame](http://semanticweb.org/wiki/Khalid_Belhajjame), University of Manchester, UK
:   [James Cheney](http://homepages.inf.ed.ac.uk/jcheney/), University of Edinburgh, UK
:   [David Corsar](http://homepages.abdn.ac.uk/dcorsar/pages/), University of Aberdeen, UK
:   [Daniel Garijo](http://www.oeg-upm.net/index.php/en/phdstudents/28-dgarijo), Ontology Engineering Group, Universidad Politécnica de Madrid, Spain
:   [Stian Soiland-Reyes](http://soiland-reyes.com/stian/), University of Manchester, UK
:   [Stephan Zednik](http://tw.rpi.edu/web/person/StephanZednik), Rensselaer Polytechnic Institute, USA
:   [Jun Zhao](http://users.ox.ac.uk/~zool0770/), University of Oxford, UK

Please refer to the [**errata**](https://www.w3.org/2011/prov/errata.html) for this document, which may include some normative corrections.

The English version of this specification is the only normative version. Non-normative [translations](https://www.w3.org/Consortium/Translation/) may also be available.

[Copyright](https://www.w3.org/Consortium/Legal/ipr-notice#Copyright) © 2011-2013 [W3C](https://www.w3.org/)® ([MIT](http://www.csail.mit.edu/), [ERCIM](http://www.ercim.eu/), [Keio](http://www.keio.ac.jp/), [Beihang](http://ev.buaa.edu.cn/)), All Rights Reserved. W3C [liability](https://www.w3.org/Consortium/Legal/ipr-notice#Legal_Disclaimer), [trademark](https://www.w3.org/Consortium/Legal/ipr-notice#W3C_Trademarks) and [document use](https://www.w3.org/Consortium/Legal/copyright-documents) rules apply.

---

Abstract
--------

The PROV Ontology (PROV-O) expresses the PROV Data Model [[PROV-DM](#bib-PROV-DM)] using the OWL2 Web Ontology Language (OWL2) [[OWL2-OVERVIEW](#bib-OWL2-OVERVIEW)]. It provides a set of classes, properties, and restrictions that can be used to represent and interchange provenance information generated in different systems and under different contexts.
It can also be specialized to create new classes and properties to model provenance information for different applications and domains.
The [PROV Document Overview](https://www.w3.org/TR/2012/WD-prov-overview-20121211/) describes the overall state of PROV, and should be read before other PROV documents.

The namespace for all PROV-O terms is http://www.w3.org/ns/prov#.

The OWL encoding of the PROV Ontology is available [here](https://www.w3.org/ns/prov-o).

Status of This Document
-----------------------

*This section describes the status of this document at the time of its publication. Other
documents may supersede this document. A list of current W3C publications and the latest revision
of this technical report can be found in the [W3C technical reports
index](https://www.w3.org/TR/) at http://www.w3.org/TR/.*

#### PROV Family of Documents

This document is part of the PROV family of documents, a set of documents defining various aspects that are necessary to achieve the vision of inter-operable
interchange of provenance information in heterogeneous environments such as the Web. These documents are listed below. Please consult the [[PROV-OVERVIEW](#bib-PROV-OVERVIEW)] for a guide to reading these documents.

* [PROV-OVERVIEW](https://www.w3.org/TR/2013/NOTE-prov-overview-20130430/) (Note), an overview of the PROV family of documents [[PROV-OVERVIEW](#bib-PROV-OVERVIEW)];
* [PROV-PRIMER](https://www.w3.org/TR/2013/NOTE-prov-primer-20130430/) (Note), a primer for the PROV data model [[PROV-PRIMER](#bib-PROV-PRIMER)];
* [PROV-O](https://www.w3.org/TR/2013/REC-prov-o-20130430/) (Recommendation), the PROV ontology, an OWL2 ontology allowing the mapping of the PROV data model to RDF (this document);
* [PROV-DM](https://www.w3.org/TR/2013/REC-prov-dm-20130430/) (Recommendation), the PROV data model for provenance [[PROV-DM](#bib-PROV-DM)];
* [PROV-N](https://www.w3.org/TR/2013/REC-prov-n-20130430/) (Recommendation), a notation for provenance aimed at human consumption [[PROV-N](#bib-PROV-N)];
* [PROV-CONSTRAINTS](https://www.w3.org/TR/2013/REC-prov-constraints-20130430/) (Recommendation), a set of constraints applying to the PROV data model [[PROV-CONSTRAINTS](#bib-PROV-CONSTRAINTS)];
* [PROV-XML](https://www.w3.org/TR/2013/NOTE-prov-xml-20130430/) (Note), an XML schema for the PROV data model [[PROV-XML](#bib-PROV-XML)];
* [PROV-AQ](https://www.w3.org/TR/2013/NOTE-prov-aq-20130430/) (Note), mechanisms for accessing and querying provenance [[PROV-AQ](#bib-PROV-AQ)];
* [PROV-DICTIONARY](https://www.w3.org/TR/2013/NOTE-prov-dictionary-20130430/) (Note) introduces a specific type of collection, consisting of key-entity pairs [[PROV-DICTIONARY](#bib-PROV-DICTIONARY)];
* [PROV-DC](https://www.w3.org/TR/2013/NOTE-prov-dc-20130430/) (Note) provides a mapping between PROV-O and Dublin Core Terms [[PROV-DC](#bib-PROV-DC)];
* [PROV-SEM](https://www.w3.org/TR/2013/NOTE-prov-sem-20130430/) (Note), a declarative specification in terms of first-order logic of the PROV data model [[PROV-SEM](#bib-PROV-SEM)];
* [PROV-LINKS](https://www.w3.org/TR/2013/NOTE-prov-links-20130430/) (Note) introduces a mechanism to link across bundles [[PROV-LINKS](#bib-PROV-LINKS)].

#### Endorsed By W3C

This document has been reviewed by W3C Members, by software developers, and by other W3C groups and interested parties, and is endorsed by the Director as a W3C Recommendation. It is a stable document and may be used as reference material or cited from another document. W3C's role in making the Recommendation is to draw attention to the specification and to promote its widespread deployment. This enhances the functionality and interoperability of the Web.

#### Please Send Comments

This document was published by the [Provenance Working Group](https://www.w3.org/2011/prov/) as a Recommendation.
If you wish to make comments regarding this document, please send them to
[public-prov-comments@w3.org](mailto:public-prov-comments@w3.org)
([subscribe](mailto:public-prov-comments-request@w3.org?subject=subscribe),
[archives](http://lists.w3.org/Archives/Public/public-prov-comments/)).
All comments are welcome.

This document was produced by a group operating under the
[5 February 2004 W3C Patent Policy](https://www.w3.org/Consortium/Patent-Policy-20040205/).
W3C maintains a [public list of any patent disclosures](https://www.w3.org/2004/01/pp-impl/46974/status)
made in connection with the deliverables of the group; that page also includes instructions for
disclosing a patent. An individual who has actual knowledge of a patent which the individual believes contains
[Essential Claim(s)](https://www.w3.org/Consortium/Patent-Policy-20040205/#def-essential) must disclose the
information in accordance with [section
6 of the W3C Patent Policy](https://www.w3.org/Consortium/Patent-Policy-20040205/#sec-Disclosure).

Table of Contents
-----------------

* [1. Introduction](#introduction)
  + [1.1 Compliance with this Document](#compliance)
  + [1.2 Notational Conventions](#notations)
  + [1.3 Namespaces](#namespaces)
* [2. PROV-O at a glance](#prov-o-at-a-glance)
* [3. The PROV-O Ontology Description](#description)
  + [3.1 Starting Point Terms](#description-starting-point-terms)
  + [3.2 Expanded Terms](#description-expanded-terms)
  + [3.3 Qualified Terms](#description-qualified-terms)
* [4. Cross reference for PROV-O classes and properties](#cross-reference)
  + [4.1 Starting Point Terms](#cross-reference-starting-point-terms)
  + [4.2 Expanded Terms](#cross-reference-expanded-terms)
  + [4.3 Qualified Terms](#cross-reference-qualified-terms)
  + [4.4 Term Index](#cross-reference-index)
* [A. PROV-O OWL Profile](#owl-profile)
* [B. Names of inverse properties](#inverse-names)
* [C. Changes since WD-prov-o-20120724](#changes-since-wd-prov-o-20120724)
* [D. Changes since CR-prov-o-20121211](#changes-since-cr-prov-o-20121211)
* [E. Changes since PR-prov-o-20130312](#changes-since-pr-prov-o-20130312)
* [F. Acknowledgements](#acknowledgements)
* [G. References](#references)
  + [G.1 Normative references](#normative-references)
  + [G.2 Informative references](#informative-references)

1. Introduction
---------------

The PROV Ontology (PROV-O) defines the OWL2 Web Ontology Language encoding of the PROV Data Model [[PROV-DM](#bib-PROV-DM)]. This document describes the set of classes, properties, and restrictions that constitute the PROV Ontology. This ontology specification provides the foundation to implement provenance applications in different domains that can represent, exchange, and integrate provenance information generated in different systems and under different contexts. Together with the PROV Access and Query [[PROV-AQ](#bib-PROV-AQ)] and PROV Data Model [[PROV-DM](#bib-PROV-DM)], this document forms a framework for provenance information interchange in domain-specific Web-based applications.

PROV-O is a lightweight ontology that can be adopted in a wide range of applications.
With the exception of [five axioms](#owl-profile), PROV-O conforms to the OWL-RL profile [[OWL2-PRIMER](#bib-OWL2-PRIMER)].
The PROV Ontology classes and properties are defined such that they can not only be used directly to represent provenance information,
but also can be specialized for modeling application-specific provenance details in a variety of domains.
Thus, the PROV Ontology is expected to be both directly usable in applications as well as serve as a *reference model*
for creating domain-specific provenance ontologies and thereby facilitates interoperable provenance modeling.
To demonstrate the use of PROV-O classes and properties, this document uses an example provenance scenario similar to the one introduced in the PROV-Primer [[PROV-PRIMER](#bib-PROV-PRIMER)].

The PROV Data Model [[PROV-DM](#bib-PROV-DM)] introduces a set of concepts to represent provenance information in a variety of application domains. This document maps the PROV Data Model to PROV Ontology using the OWL2 ontology language [[OWL2-OVERVIEW](#bib-OWL2-OVERVIEW)].

We briefly introduce some of the OWL2 modeling terms that will be used to describe the PROV Ontology.
An OWL2 *instance* is an individual object in a domain of discourse, for example a person named Alice or a car named KITT.
A set of individuals sharing common characteristics constitutes a *class*.
Person and Car are examples of classes representing the set of individual persons and cars respectively.
The OWL2 object properties are used to link individuals, classes, or create a property hierarchy.
For example, the object property "hasOwner" can be used to link car with person.
The OWL2 datatype properties are used to link individuals or classes to data values, including XML Schema datatypes [[XMLSCHEMA11-2](#bib-XMLSCHEMA11-2)].

### 1.1 Compliance with this Document

For the purpose of compliance, the normative sections of this document are
[Section 1.1](#compliance),
[Section 1.2](#notations),
[Section 3](#description),
[Section 4](#cross-reference), and
[Appendix B](#inverse-names)

* Information in tables is normative if it appears in a normative section.
* All figures and diagrams are informative.
* All examples are informative.

### 1.2 Notational Conventions

The key words "*must*", "*must not*", "*required*", "*shall*", "*shall not*", "*should*", "*should not*", "*recommended*", "*may*", and "*optional*" in this document are to be interpreted as described in [[RFC2119](#bib-RFC2119)].

### 1.3 Namespaces

*This section is non-normative.*

The following namespace prefixes are used throughout this document.

[Table 1](#namespace-table): Prefix and Namespaces used in this specification

|  |  |  |
| --- | --- | --- |
| **prefix** | **namespace IRI** | **definition** |
| rdf | http://www.w3.org/1999/02/22-rdf-syntax-ns# | The RDF namespace [[RDF-CONCEPTS](#bib-RDF-CONCEPTS)] |
| xsd | http://www.w3.org/2000/10/XMLSchema# | XML Schema Namespace [[XMLSCHEMA11-2](#bib-XMLSCHEMA11-2)] |
| owl | http://www.w3.org/2002/07/owl# | The OWL namespace [[OWL2-OVERVIEW](#bib-OWL2-OVERVIEW)] |
| prov | http://www.w3.org/ns/prov# | The PROV namespace [[PROV-DM](#bib-PROV-DM)] |
| (others) | (various) | All other namespace prefixes are used in examples only.   In particular, IRIs starting with "http://example.com" represent  some application-dependent IRI [[IRI](#bib-IRI)] |

2. PROV-O at a glance
---------------------

*This section is non-normative.*

PROV-O users may only need to use parts of the entire ontology, depending on their needs and according to how much detail they want to include in their provenance information. For this, the PROV-O terms (classes and properties) are grouped into three categories to provide an incremental introduction to the ontology: Starting Point terms, Expanded terms, and terms for Qualifying relationships.

**Starting Point classes and properties** provide the basis for the rest of the PROV Ontology and thus it is recommended that readers become comfortable with how to apply these terms before continuing to the remaining categories. These terms are used to create simple provenance descriptions that can be elaborated using terms from other categories. The classes and properties in this category are listed below and are discussed in [Section 3.1](#description-starting-point-terms).

* [prov:Entity](#Entity)
* [prov:Activity](#Activity)
* [prov:Agent](#Agent)

* [prov:wasGeneratedBy](#wasGeneratedBy)
* [prov:wasDerivedFrom](#wasDerivedFrom)
* [prov:wasAttributedTo](#wasAttributedTo)
* [prov:startedAtTime](#startedAtTime)
* [prov:used](#used)
* [prov:wasInformedBy](#wasInformedBy)
* [prov:endedAtTime](#endedAtTime)
* [prov:wasAssociatedWith](#wasAssociatedWith)
* [prov:actedOnBehalfOf](#actedOnBehalfOf)

**Expanded classes and properties** provide additional terms that can be used to relate classes in the Starting Point category. The terms in this category are applied in the same way as the terms in the Starting Point category. Many of the terms in this category are subclasses or subproperties of those in the Starting Point category. The classes and properties in this category are listed below and are discussed in [Section 3.2](#description-expanded-terms).

* [prov:Collection](#Collection)
* [prov:EmptyCollection](#EmptyCollection)
* [prov:Bundle](#Bundle)
* [prov:Person](#Person)
* [prov:SoftwareAgent](#SoftwareAgent)
* [prov:Organization](#Organization)
* [prov:Location](#Location)

* [prov:alternateOf](#alternateOf)
* [prov:specializationOf](#specializationOf)
* [prov:generatedAtTime](#generatedAtTime)
* [prov:hadPrimarySource](#hadPrimarySource)
* [prov:value](#value)
* [prov:wasQuotedFrom](#wasQuotedFrom)
* [prov:wasRevisionOf](#wasRevisionOf)
* [prov:invalidatedAtTime](#invalidatedAtTime)
* [prov:wasInvalidatedBy](#wasInvalidatedBy)
* [prov:hadMember](#hadMember)
* [prov:wasStartedBy](#wasStartedBy)
* [prov:wasEndedBy](#wasEndedBy)
* [prov:invalidated](#invalidated)
* [prov:influenced](#influenced)
* [prov:atLocation](#atLocation)
* [prov:generated](#generated)

**Qualified classes and properties** provide elaborated information about binary relations asserted using Starting Point and Expanded properties. The terms in this category are applied using a pattern that differs from those in the Starting Point and Expanded categories. While the relations from the previous two categories are applied as direct, binary assertions, the terms in this category are used to provide additional attributes of the binary relations. The pattern used in this category allows users to provide elaborate details that are not available using only Starting Point and Expanded terms. The classes and properties in this category are listed below and are discussed in [Section 3.3](#description-qualified-terms).

* [prov:Influence](#Influence)
* [prov:EntityInfluence](#EntityInfluence)
* [prov:Usage](#Usage)
* [prov:Start](#Start)
* [prov:End](#End)
* [prov:Derivation](#Derivation)
* [prov:PrimarySource](#PrimarySource)
* [prov:Quotation](#Quotation)
* [prov:Revision](#Revision)
* [prov:ActivityInfluence](#ActivityInfluence)
* [prov:Generation](#Generation)
* [prov:Communication](#Communication)
* [prov:Invalidation](#Invalidation)
* [prov:AgentInfluence](#AgentInfluence)
* [prov:Attribution](#Attribution)
* [prov:Association](#Association)
* [prov:Plan](#Plan)
* [prov:Delegation](#Delegation)
* [prov:InstantaneousEvent](#InstantaneousEvent)
* [prov:Role](#Role)

* [prov:wasInfluencedBy](#wasInfluencedBy)
* [prov:qualifiedInfluence](#qualifiedInfluence)
* [prov:qualifiedGeneration](#qualifiedGeneration)
* [prov:qualifiedDerivation](#qualifiedDerivation)
* [prov:qualifiedPrimarySource](#qualifiedPrimarySource)
* [prov:qualifiedQuotation](#qualifiedQuotation)
* [prov:qualifiedRevision](#qualifiedRevision)
* [prov:qualifiedAttribution](#qualifiedAttribution)
* [prov:qualifiedInvalidation](#qualifiedInvalidation)
* [prov:qualifiedStart](#qualifiedStart)
* [prov:qualifiedUsage](#qualifiedUsage)
* [prov:qualifiedCommunication](#qualifiedCommunication)
* [prov:qualifiedAssociation](#qualifiedAssociation)
* [prov:qualifiedEnd](#qualifiedEnd)
* [prov:qualifiedDelegation](#qualifiedDelegation)
* [prov:influencer](#influencer)
* [prov:entity](#p_entity)
* [prov:hadUsage](#hadUsage)
* [prov:hadGeneration](#hadGeneration)
* [prov:activity](#p_activity)
* [prov:agent](#p_agent)
* [prov:hadPlan](#hadPlan)
* [prov:hadActivity](#hadActivity)
* [prov:atTime](#atTime)
* [prov:hadRole](#hadRole)

3. The PROV-O Ontology Description
----------------------------------

This section introduces the terms in each of the following categories:

* [Starting Point Terms](#description-starting-point-terms)
* [Expanded Terms](#description-expanded-terms)
* [Qualified Terms](#description-qualified-terms)

### 3.1 Starting Point Terms

The Starting Point category is a small set of classes and properties that can be used to create simple, initial provenance descriptions.
Three classes provide a basis for the rest of PROV-O:

* An [prov:Entity](#Entity) is a physical, digital, conceptual, or other kind of thing with some fixed aspects; entities may be real or imaginary.
* An [prov:Activity](#Activity) is something that occurs over a period of time and acts upon or with entities; it may include consuming, processing, transforming, modifying, relocating, using, or generating entities.
* An [prov:Agent](#Agent) is something that bears some form of responsibility for an activity taking place, for the existence of an entity, or for another agent's activity.

The three primary classes relate to one another and to themselves using the properties shown in the following figure.

Activities **start** and **end** at particular points in time (described using properties
[prov:startedAtTime](#startedAtTime) and [prov:endedAtTime](#endedAtTime), respectively)
and during their lifespan can **use** and **generate** a variety of Entities (described with
[prov:used](#used) and [prov:wasGeneratedBy](#wasGeneratedBy), respectively).
For example, a blog writing activity may use a particular dataset and generate a bar chart.
By expressing usage and generation, one can construct provenance chains comprising both Activities and Entities.

In addition, we can say that an Activity [prov:wasInformedBy](#wasInformedBy)
another Activity to provide some dependency information without explicitly providing the activities' start and end times.
A prov:wasInformedBy relation between Activities suggests that the informed Activity used an Entity that was generated by the informing
Activity, but the Entity itself is unknown or is not of interest.
So, the prov:wasInformedBy property allows the construction of provenance chains comprising only Activities.

Provenance chains comprising only Entities can be formed using the [prov:wasDerivedFrom](#wasDerivedFrom) property.
A derivation is a transformation of one entity into another. For example, if the Activity that created the bar chart is not known or is not of interest,
then we can say that the bar chart prov:wasDerivedFrom the dataset.
Arbitrary RDF properties can be used to describe the fixed aspects of an Entity that are interesting within a particular application (for example,
the file size and format of the dataset, or the aspect ratio of the bar chart).

While the properties prov:used, prov:wasGeneratedBy,
prov:wasInformedBy, and prov:wasDerivedFrom can be used to construct provenance chains among
Activities and Entities, Agents may also be ascribed responsibility for any Activity or Entity within a provenance chain.
An Agent's responsibility for an Activity or Entity is described using the properties [prov:wasAssociatedWith](#wasAssociatedWith) and
[prov:wasAttributedTo](#wasAttributedTo), respectively.
Agents can also be responsible for other Agents' actions. In this case of delegation, the influencing Agent [prov:actedOnBehalfOf](#actedOnBehalfOf)
another Agent that also bears responsibility for the influenced Activity or Entity.

The properties rdf:type and rdfs:label are used to express [prov:type](https://dvcs.w3.org/hg/prov/raw-file/default/model/prov-dm.html#term-attribute-type)
and [prov:label](https://dvcs.w3.org/hg/prov/raw-file/default/model/prov-dm.html#term-attribute-label), respectively.

![PROV-O Starting Point terms](diagrams/starting-points.svg)

[Figure 1](#starting-points-figure).
The three Starting Point classes and the properties that relate them.  
The diagrams in this document depict Entities as yellow ovals,  
Activities as blue rectangles, and Agents as orange pentagons.  
The responsibility properties are shown in pink.

[Example 1](#narrative-example-simple-1): The following PROV-O describes the resources involved when creating a chart about crime statistics. The example uses only Starting Point terms and serves as a basis for elaboration that will be described in subsequent sections. In the example, Derek performs an aggregation of some government crime data, grouping by national regions that are described in a separate dataset by a civil action group.

```
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.org#> .

:bar_chart
   a prov:Entity;
   prov:wasGeneratedBy  :illustrationActivity;
   prov:wasDerivedFrom  :aggregatedByRegions;
   prov:wasAttributedTo :derek;
.

:derek
   a foaf:Person, prov:Agent;
   foaf:givenName       "Derek";
   foaf:mbox            <mailto:derek@example.org>;
   prov:actedOnBehalfOf :natonal_newspaper_inc;
.

:national_newspaper_inc 
   a foaf:Organization, prov:Agent;
   foaf:name "National Newspaper, Inc.";
.

:illustrationActivity 
   a prov:Activity; 
   prov:used              :aggregatedByRegions;
   prov:wasAssociatedWith :derek;
   prov:wasInformedBy     :aggregationActivity;
.

:aggregatedByRegions
   a prov:Entity;
   prov:wasGeneratedBy  :aggregationActivity;
   prov:wasAttributedTo :derek;
.

:aggregationActivity
   a prov:Activity;
   prov:startedAtTime    "2011-07-14T01:01:01Z"^^xsd:dateTime;
   prov:wasAssociatedWith :derek;
   prov:used              :crimeData;
   prov:used              :nationalRegionsList;
   prov:endedAtTime      "2011-07-14T02:02:02Z"^^xsd:dateTime;
.

:crimeData
   a prov:Entity;
   prov:wasAttributedTo :government;
.
:government a foaf:Organization, prov:Agent .

:nationalRegionsList 
   a prov:Entity;
   prov:wasAttributedTo :civil_action_group;
.
:civil_action_group a foaf:Organization, prov:Agent .
```

The example states that the agent `:derek` was associated with two
activities: `:aggregationActivity` and `:illustrationActivity`. The
activity `:aggregationActivity` used
the entities `:crimeData` (a crime statistics dataset) and `:nationalRegionsList` (a list of national regions), and
generated a new entity, `:aggregatedByRegions` that aggregates the statistics in
`:crimeData` according to the regions in `:nationalRegionsList`.
The `:aggregatedByRegions` entity was then used by the `:illustrationActivity` activity,
to generate a new entity `:bar_chart` that depicts the aggregated statistics.

The example also states that the activity `:illustrationActivity` was
informed by the activity `:aggregationActivity`. Indeed, the former used
the entity `:aggregatedByRegions`, which was generated by the latter.

Because the agent `:derek` was associated with the activities
`:aggregationActivity` and `:illustrationActivity`, the entities
generated by these activities, i.e., `:aggregatedByRegions` and `:bar_chart`, were also
attributed to him.

Finally, the example states that the agent `:derek` acted on behalf of the organization `:national_newspaper_inc`.

![Illustration of Derek making a bar chart from some aggregated government data.](diagrams/starting-points-example.svg)

[Figure 2](#narrative-example-simple-1-figure).
A graphical illustration of the PROV-O in [Example 1](#narrative-example-simple-1),
showing how the three Starting Point classes relate.  
The diagrams in this document depict Entities as yellow ovals, Activities as blue rectangles,  
and Agents as orange pentagons. The responsibility properties are shown in pink.

### 3.2 Expanded Terms

The terms introduced in this section provide additional ways to describe the provenance among Entities, Activities, and Agents.
The additional terms are illustrated in the following figure and can be separated into five different categories.

![PROV-O Starting Point terms](diagrams/expanded.svg)

[Figure 3](#expanded-terms-figure).
The expanded terms build upon those in the [Starting Points section](#description-starting-point-terms).  
The diagrams in this document depict Entities as yellow ovals, Activities as blue rectangles, and Agents as orange pentagons.  
The domain of [prov:atLocation](#atLocation) (prov:Activity or prov:Entity or prov:Agent or prov:InstantaneousEvent) is not illustrated.

The **first** category extends the [Starting Point terms](#description-starting-point-terms) with subclasses, subproperties, and a superproperty.

Three subclasses of Agent ([prov:Person](#Person), [prov:Organization](#Organization), and
[prov:SoftwareAgent](#SoftwareAgent)) and
three subclasses of Entity are provided ([prov:Collection](#Collection), [prov:Bundle](#Bundle),
and [prov:Plan](#Plan)).

A prov:Collection is an Entity that provides a structure (e.g. set, list, etc.) to some constituents (which are themselves Entities).
The prov:Collection class can be used to express the provenance of the collection itself:
e.g. who maintained the collection, which members it contained as it evolved, and how it was assembled.
The [prov:hadMember](#hadMember) property is used to assert membership in a collection.

A prov:Bundle is a named set of provenance descriptions, which may itself have provenance.
The named set of provenance descriptions may be expressed as PROV-O or any other form.
The subclass of Bundle that names a set of PROV-O assertions is not provided by PROV-O, since it is more appropriate to do so using other recommendations,
standards, or technologies. In any case, a Bundle of PROV-O assertions is an abstract set of RDF triples, and adding or removing a triple creates a new distinct
Bundle of PROV-O assertions.

A prov:Plan is an entity that represents a set of actions or steps intended by one or more agents to achieve some goals.

More general and more specific properties are also provided by the expanded terms. More generally, the property
[prov:wasInfluencedBy](#wasInfluencedBy) is a superproperty that relates any influenced Entity, Activity, or Agent to any other
influencing Entity, Activity, or Agent that had an effect on its characteristics.
Three subproperties of prov:wasDerivedFrom are also provided for certain kinds of derivation among Entities:
[prov:wasQuotedFrom](#wasQuotedFrom) cites a potentially larger Entity (such as a book, blog, or image) from which a new Entity was created
by repeating some or all of the original,
[prov:wasRevisionOf](#wasRevisionOf) indicates that the derived Entity contains substantial content from the original Entity
(e.g., two editions of a book), and
[prov:hadPrimarySource](#hadPrimarySource) cites a preceding Entity produced by some agent with direct experience and
knowledge about the topic (such as a reading from a sensor, or a journal written during an historical event).

The **second** category of expanded terms relates Entities according to their levels of abstraction, where some Entities may present more specific aspects than their more general counterparts.
While [prov:specializationOf](#specializationOf) links a more specific Entity to a more general one (e.g., today's BBC news home page versus BBC's news home page on any day), [prov:alternateOf](#alternateOf) links Entities that present aspects of the same thing, but not necessarily the same aspects or at the same time (e.g., the serialization of a document in different formats or a backup copy of a computer file).

The **third** category of expanded terms allows further description of Entities. The property [prov:value](#value)
provides a literal value that is a direct representation of an entity.
For example, the prov:value of a quote could be a string of the sentences stated, or the prov:value of an Entity involved in a numeric calculation could be the xsd:integer four.
The property [prov:atLocation](#atLocation) can be used to describe the [prov:Location](#Location) of any
Entity, Activity, Agent, or [prov:InstantaneousEvent](#InstantaneousEvent)
(i.e., the starting or ending of an activity or the generation, usage, or invalidation of an entity).
The properties used to describe instances of prov:Location are outside the scope of PROV-O;
reuse of other existing vocabulary is encouraged.

The **fourth** category of expanded terms describes the lifetime of an Entity beyond being **generated** by an Activity and **used** by other Activities. For example, a painting could not have been displayed before it was painted, and it could not be sold after it was destroyed by fire.
Similar to how Activities have start and end times, an Entity may be bound by points in time for which it was generated or is no longer usable.
The properties [prov:generatedAtTime](#generatedAtTime) and [prov:invalidatedAtTime](#invalidatedAtTime) can be used to bound the starting and ending moments of an Entity's existence. The Activities that led to the generation or invalidation of an Entity can be provided using prov:wasGeneratedBy and [prov:wasInvalidatedBy](#wasInvalidatedBy), respectively.
[prov:generated](#generated) and [prov:invalidated](#invalidated) are the inverses of prov:wasGeneratedBy and prov:wasInvalidatedBy, respectively, and are defined to facilitate Activity-as-subject as well as Entity-as-subject descriptions.
For more about inverses, see the [Appendix B](#inverse-names).

The **fifth** category of expanded terms describes the lifetime of an Activity beyond its start and end times and predecessor Activities.
Activities may also be started or ended by Entities, which are described using the properties [prov:wasStartedBy](#wasStartedBy)
and [prov:wasEndedBy](#wasEndedBy), respectively. Since Entities may start or end Activities, and Agents may be Entities,
then Agents may also start or end Activities.

The following examples illustrate the expanded terms by elaborating the [crime chart example](#narrative-example-simple-1) from the previous section.
After aggregating the dataset and creating the chart, Derek published a post to exhibit his work.

[Example 2](#narrative-example-expanded-1):

```
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix sioc: <http://rdfs.org/sioc/ns#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix my:   <http://www.example.org/vocab#> .
@prefix :     <http://www.example.org#> .
@base         <http://www.example.com/derek-bundle.ttl> .

<> 
   a prov:Bundle, prov:Entity;
   prov:wasAttributedTo :postEditor;
   prov:generatedAtTime "2011-07-16T02:52:02Z"^^xsd:dateTime;
.

:derek
   a prov:Person, prov:Agent; ## prov:Agent is inferred from prov:Person
   foaf:givenName       "Derek";
   foaf:mbox            <mailto:derek@example.org>;
   prov:actedOnBehalfOf :national_newspaper_inc;
.

:national_newspaper_inc 
   a prov:Organization, prov:Agent;        ## prov:Agent is inferred from prov:Organization
   foaf:name "National Newspaper, Inc.";
.

:postEditor 
   a prov:SoftwareAgent, prov:Agent;       ## prov:Agent is inferred from prov:SoftwareAgent
   foaf:name "Post Editor 3000";
.   

:more-crime-happens-in-cities
   a sioc:Post, prov:Entity;
   sioc:latest_version   :post9821v2;
   sioc:previous_version :post9821v1;
.

## Version 1 of the post

:post9821v1
   a sioc:Post, prov:Entity;   
   prov:specializationOf  :more-crime-happens-in-cities; ## PERMALINK to the latest revision.
   sioc:title             "More crime happens in cities";
   prov:value             "I was currius...";            ## The text of this version (with a typo).
   prov:generatedAtTime   "2011-07-16T01:52:02Z"^^xsd:dateTime;
   prov:wasGeneratedBy    :publicationActivity1123;
   prov:wasInfluencedBy   :aggregatedByRegions; ## This blog was influenced by Derek's data analysis.
   prov:hadPrimarySource  :crimeData;           ## Derek's blog derives from the crime data originally
                                                ## gathered by the government.
   prov:invalidatedAtTime "2011-07-16T02:02:02Z"^^xsd:dateTime;
.

:publicationActivity1123 
   a prov:Activity;
   prov:startedAtTime     "2011-07-16T01:01:01Z"^^xsd:dateTime;
   prov:wasStartedBy      :derek;
   prov:wasAssociatedWith :postEditor;
   prov:generated         :post9821v1;
   prov:endedAtTime       "2011-07-16T01:52:02Z"^^xsd:dateTime;
   prov:wasEndedBy        :derek;
.

:aggregatedByRegions
   a prov:Entity;
   prov:atLocation <file://Users/aggr.txt>;
.
<file://Users/aggr.txt> a prov:Location .

:crimeData
   a prov:Entity;
   prov:wasAttributedTo :government;
.
:government 
   a prov:Organization, prov:Agent;
.

## Version 2 of the post

:post9821v2
   a sioc:Post, prov:Entity;
   prov:specializationOf :more-crime-happens-in-cities; ## PERMALINK to the latest revision.
   prov:value            "I was curious...";            ## The text of this version (with fixed typo).
   prov:generatedAtTime  "2011-07-16T02:02:02Z"^^xsd:dateTime;
   prov:wasRevisionOf    :post9821v1;
   prov:alternateOf      :post9821v1;
.
```

Agent `:derek`, acting again on behalf of the `:national_newspaper_inc` organization,
used the `:postEditor` tool to publish a post about his recent data analysis `:aggregatedByRegions`.
The blog editing tool tracked Derek's actions as PROV-O assertions and published them as a Bundle (the current file `<>`).
The tool recorded that `:derek` started and ended the publishing activity (`:publicationActivity1123`)
that generated the post `:post9821v1`. The post
included a permanent link where the content of the latest version is available
(`:more-crime-happens-in-cities`) in addition to a textual snapshot of the current version (using prov:value).
Derek also included additional domain-specific descriptions of the post, such as its title.

Shortly after publishing the post, Derek noticed a typographical error in his narrative.
Because the fix would be minimal, he did not record the activity that led to the new version.
Instead, he related the new version (`:post9821v2`) as a revision of the previous (`:post9821v1`).
Since both versions of the blog are forms of the long-standing blog permalink `:more-crime-happens-in-cities`,
the revisions are alternates of one another and each is a [prov:specializationOf](#specializationOf) of `:more-crime-happens-in-cities`.

![PROV-O Starting Point terms](diagrams/expanded-terms-example-bundlePost.svg)

[Figure 4](#expanded-example-illustration-1).
An illustration of the PROV-O assertions in [Example 2](#narrative-example-expanded-1), where Derek  
published two versions of a blog for the National Newspaper, Inc.  
The diagrams in this document depict Entities as yellow ovals, Activities as blue rectangles,  
and Agents as orange pentagons. The responsibility properties are shown in pink.

Shortly after Derek published his blog post, Monica adapted the text for a wider audience in a new post (`:post9822`).
This rewrite is an alternate, abbreviated view of the same topic that Derek wrote about and was created from his original text.
Since the provenance produced by the activities of Derek and Monica corresponded to different user views, the system
automatically published it in a different [prov:Bundle](#Bundle).
The tool also asserted provenance about the bundle that it produced (e.g., the date of creation, its creator, and the fact that it Derek's bundle was used).
Because a bundle is a kind of entity, all provenance assertions that can be made about entities can also be made about bundles.
The use of bundles enables the creation of provenance of provenance.

[Example 3](#narrative-example-expanded-2):

```
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix sioc: <http://rdfs.org/sioc/ns#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://www.example.org#> .
@base         <http://www.example.com/monica-bundle.ttl> .

<>
   a prov:Bundle, prov:Entity;
   prov:wasAttributedTo :postEditor;
   prov:wasDerivedFrom  <http://www.example.com/derek-bundle.ttl> .
   prov:generatedAtTime "2011-07-16T03:03:03Z"^^xsd:dateTime;
.

:monica
   a prov:Person, prov:Agent;
   foaf:givenName "Monica";
   foaf:mbox      <mailto:monica@example.org>;
.

## Revised post for a different audience is a new resource.

:post9822
   a sioc:Post, prov:Entity;
   sioc:title "More crime happens in cities (for dummies)";
   prov:wasAttributedTo  :monica,
                         :postEditor;
   prov:alternateOf      :more-crime-happens-in-cities;    ## This post is an alternate of Derek's blog.
   prov:value            "A quick overview of Derek's..."; ## Snapshot with the content of this version
   prov:wasRevisionOf    :post9821v2;                      ## Monica rewrote Derek's version 2 to create hers.
.
```

After some time, John wrote his own conclusions in his own post (`:post19201`) quoting the previous two posts.
Each quote that John makes (`:quote_from_monica` and `:quote_from_derek`) is a new entity derived from the
previous blogs and is annotated with the time that the quote was taken.
The provenance of John's blog notes that his post is the result of the quotes that he took from Derek and Monica.
The blog post is also derived from Derek's `:aggregatedByRegions` dataset because John inspected it and found a
concern that he discusses in his blog. All the provenance statements related to John's post are grouped in a new prov:Bundle.

[Example 4](#narrative-example-expanded-3):

```
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix sioc: <http://rdfs.org/sioc/ns#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://www.example.org#> .
@base         <http://www.example.com/john-bundle.ttl> .

<> 
   a prov:Bundle, prov:Entity;
   prov:wasAttributedTo :postEditor;
   prov:wasDerivedFrom <http://www.example.com/derek-bundle.ttl>,
                       <http://www.example.com/monica-bundle.ttl>;
   prov:generatedAtTime "2012-08-08T08:08:08Z"^^xsd:dateTime;
.

:publicationActivity1124
   a prov:Activity;
   prov:wasAttributedTo :postEditor,
                        :john;
   prov:generated :post19201;
.

:post19201
   a sioc:Post, prov:Entity;
   prov:wasAttributedTo :john;
   prov:value "I'm not so sure that...";
   prov:wasDerivedFrom :quote_from_derek,
                       :quote_from_monica,
                       :aggregatedByRegions;
   prov:wasGeneratedBy :publicationActivity1124;
.

:john 
   a prov:Person, prov:Agent;
   foaf:name "John";
.

:quote_from_derek
   a prov:Entity;
   prov:value "Analysis of the datasets demonstrates that there is more crime.";
   prov:wasQuotedFrom   :more-crime-happens-in-cities;
   prov:generatedAtTime "2012-08-08T01:01:01Z"^^xsd:dateTime;
.
   
:quote_from_monica
   a prov:Entity;
   prov:value "In summary, there are clearly more crimes in the country.";
   prov:wasQuotedFrom   :post9822;
   prov:generatedAtTime "2012-08-08T02:02:02Z"^^xsd:dateTime;
.
```

Unfortunately, there was a problem in the servers where `:post19201` was being stored, and all the data related to the post was lost permanently.
Thus, the system invalidated the entity automatically and notified John about the error.

[Example 5](#narrative-example-expanded-4):

```
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix sioc: <http://rdfs.org/sioc/ns#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.org#> .

:post19201
   a sioc:Post, prov:Entity;
   prov:invalidatedAtTime "2012-09-02T01:31:00Z"^^xsd:dateTime;
   prov:wasInvalidatedBy :hard_disk_failure;
.

:hard_disk_failure
   a prov:Activity;
   prov:endedAtTime "2012-09-02T01:31:00Z"^^xsd:dateTime;
.
```

### 3.3 Qualified Terms

The Qualified Terms category is the result of
applying the **Qualification Pattern** [[LD-Patterns-QR](#bib-LD-Patterns-QR)] to the simple (unqualified)
relations available in the [Starting Point](#prov-starting-point-owl-terms-at-a-glance) and
[Expanded](#prov-expanded-owl-terms-at-a-glance) categories.
The terms in this category are for users who wish to provide further details about the provenance-related influence among
Entities, Activities, and Agents.

The Qualification Pattern restates an unqualified influence
relation by using an intermediate class that represents the influence between two resources.
This new instance, in turn, can be annotated with additional descriptions of the influence that one resource had upon another.
The following two tables list the influence relations that can be qualified using the Qualification Pattern, along with the properties used to qualify them.
For example, the second row of the first table indicates that to elaborate how an prov:Activity
prov:used a particular prov:Entity, one creates an instance of prov:Usage
that indicates the influencing entity with the prov:entity property.
Meanwhile, the influenced prov:Activity indicates the prov:Usage with the property
prov:qualifiedUsage.
The resulting structure that qualifies the an Activity's usage of an Entity is illustrated in [Figure 4a](#qualified-terms-figure) below.

Seven [Starting Point](#description-starting-point-terms) relations can be further described using the Qualification Pattern.
They are listed in the following normative table.

[Table 2](#qualified-forms-starting-point): Qualification Property and Qualified Influence Class used to qualify a Starting-point Property.

| Influenced Class | Unqualified Influence | Influencing Class | Qualification Property | Qualified Influence | Influencer Property |
| --- | --- | --- | --- | --- | --- |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:wasGeneratedBy](#wasGeneratedBy "http://www.w3.org/ns/prov#wasGeneratedBy") | [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:qualifiedGeneration](#qualifiedGeneration "http://www.w3.org/ns/prov#qualifiedGeneration") | [prov:Generation](#Generation "http://www.w3.org/ns/prov#Generation") | [prov:activity](#p_activity "http://www.w3.org/ns/prov#activity") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:wasDerivedFrom](#wasDerivedFrom "http://www.w3.org/ns/prov#wasDerivedFrom") | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:qualifiedDerivation](#qualifiedDerivation "http://www.w3.org/ns/prov#qualifiedDerivation") | [prov:Derivation](#Derivation "http://www.w3.org/ns/prov#Derivation") | [prov:entity](#p_entity "http://www.w3.org/ns/prov#entity") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:wasAttributedTo](#wasAttributedTo "http://www.w3.org/ns/prov#wasAttributedTo") | [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent") | [prov:qualifiedAttribution](#qualifiedAttribution "http://www.w3.org/ns/prov#qualifiedAttribution") | [prov:Attribution](#Attribution "http://www.w3.org/ns/prov#Attribution") | [prov:agent](#p_agent "http://www.w3.org/ns/prov#agent") |
| [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:used](#used "http://www.w3.org/ns/prov#used") | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:qualifiedUsage](#qualifiedUsage "http://www.w3.org/ns/prov#qualifiedUsage") | [prov:Usage](#Usage "http://www.w3.org/ns/prov#Usage") | [prov:entity](#p_entity "http://www.w3.org/ns/prov#entity") |
| [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:wasInformedBy](#wasInformedBy "http://www.w3.org/ns/prov#wasInformedBy") | [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:qualifiedCommunication](#qualifiedCommunication "http://www.w3.org/ns/prov#qualifiedCommunication") | [prov:Communication](#Communication "http://www.w3.org/ns/prov#Communication") | [prov:activity](#p_activity "http://www.w3.org/ns/prov#activity") |
| [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:wasAssociatedWith](#wasAssociatedWith "http://www.w3.org/ns/prov#wasAssociatedWith") | [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent") | [prov:qualifiedAssociation](#qualifiedAssociation "http://www.w3.org/ns/prov#qualifiedAssociation") | [prov:Association](#Association "http://www.w3.org/ns/prov#Association") | [prov:agent](#p_agent "http://www.w3.org/ns/prov#agent") |
| [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent") | [prov:actedOnBehalfOf](#actedOnBehalfOf "http://www.w3.org/ns/prov#actedOnBehalfOf") | [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent") | [prov:qualifiedDelegation](#qualifiedDelegation "http://www.w3.org/ns/prov#qualifiedDelegation") | [prov:Delegation](#Delegation "http://www.w3.org/ns/prov#Delegation") | [prov:agent](#p_agent "http://www.w3.org/ns/prov#agent") |

Seven [Expanded](#description-expanded-terms) relations can be further described using the Qualification Pattern.
They are listed in the following normative table.

[Table 3](#qualified-forms-expanded): Qualification Property and Qualified Influence Class used to qualify an Expanded Property.

| Influenced Class | Unqualified Influence | Influencing Class | Qualification Property | Qualified Influence | Influencer Property |
| --- | --- | --- | --- | --- | --- |
| [prov:Entity](#Entity "provEntity") or [prov:Activity](#Activity "provActivity") or [prov:Agent](#Agent "provAgent") | [prov:wasInfluencedBy](#wasInfluencedBy "provwasInfluencedBy") | [prov:Entity](#Entity "provEntity") or [prov:Activity](#Activity "provActivity") or [prov:Agent](#Agent "provAgent") | [prov:qualifiedInfluence](#qualifiedInfluence "provqualifiedInfluence") | [prov:Influence](#Influence "provInfluence") | [prov:influencer](#influencer "provinfluencer") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:hadPrimarySource](#hadPrimarySource "http://www.w3.org/ns/prov#hadPrimarySource") | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:qualifiedPrimarySource](#qualifiedPrimarySource "http://www.w3.org/ns/prov#qualifiedPrimarySource") | [prov:PrimarySource](#PrimarySource "http://www.w3.org/ns/prov#PrimarySource") | [prov:entity](#p_entity "http://www.w3.org/ns/prov#entity") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:wasQuotedFrom](#wasQuotedFrom "http://www.w3.org/ns/prov#wasQuotedFrom") | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:qualifiedQuotation](#qualifiedQuotation "http://www.w3.org/ns/prov#qualifiedQuotation") | [prov:Quotation](#Quotation "http://www.w3.org/ns/prov#Quotation") | [prov:entity](#p_entity "http://www.w3.org/ns/prov#entity") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:wasRevisionOf](#wasRevisionOf "http://www.w3.org/ns/prov#wasRevisionOf") | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:qualifiedRevision](#qualifiedRevision "http://www.w3.org/ns/prov#qualifiedRevision") | [prov:Revision](#Revision "http://www.w3.org/ns/prov#Revision") | [prov:entity](#p_entity "http://www.w3.org/ns/prov#entity") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:wasInvalidatedBy](#wasInvalidatedBy "http://www.w3.org/ns/prov#wasInvalidatedBy") | [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:qualifiedInvalidation](#qualifiedInvalidation "http://www.w3.org/ns/prov#qualifiedInvalidation") | [prov:Invalidation](#Invalidation "http://www.w3.org/ns/prov#Invalidation") | [prov:activity](#p_activity "http://www.w3.org/ns/prov#activity") |
| [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:wasStartedBy](#wasStartedBy "http://www.w3.org/ns/prov#wasStartedBy") | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:qualifiedStart](#qualifiedStart "http://www.w3.org/ns/prov#qualifiedStart") | [prov:Start](#Start "http://www.w3.org/ns/prov#Start") | [prov:entity](#p_entity "http://www.w3.org/ns/prov#entity") |
| [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:wasEndedBy](#wasEndedBy "http://www.w3.org/ns/prov#wasEndedBy") | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:qualifiedEnd](#qualifiedEnd "http://www.w3.org/ns/prov#qualifiedEnd") | [prov:End](#End "http://www.w3.org/ns/prov#End") | [prov:entity](#p_entity "http://www.w3.org/ns/prov#entity") |

The qualification classes and properties shown in the previous two tables can also be found in the normative [cross reference](#cross-reference)
in the next section of this document.
All influence classes (e.g. prov:Association, prov:Usage) are extensions of
[prov:Influence](#Influence) and either
[prov:EntityInfluence](#EntityInfluence),
[prov:ActivityInfluence](#ActivityInfluence), or
[prov:AgentInfluence](#AgentInfluence), which determine the property used to cite the influencing resource (either
[prov:entity](#p_entity),
[prov:activity](#p_activity), or
[prov:agent](#p_agent), respectively).
Because prov:Influence is a broad relation, its most specific subclasses (e.g. prov:Communication,
prov:Delegation, prov:End, prov:Revision, etc.) should be used when applicable.

[Example 6](#narrative-example-qualified-1):

For example, given the unqualified statement:

```
:e1 
   a prov:Entity;
   prov:wasGeneratedBy :a1;
.

:a1 a prov:Activity .
```

One can find that [prov:wasGeneratedBy](#wasGeneratedBy)
can be qualified using the qualification property [prov:qualifiedGeneration](#qualifiedGeneration),
the class [prov:Generation](#Generation) (a subclass of [prov:ActivityInfluence](#ActivityInfluence)),
and the property [prov:activity](#p_activity).
From this, the influence relation above can be restated with the *qualification pattern* as:

[Example 7](#narrative-example-qualified-2):

```
:e1 
   a prov:Entity;
   prov:wasGeneratedBy      :a1;
   prov:qualifiedGeneration :e1Gen; # Add the qualification.
.

:e1Gen 
   a prov:Generation;
   prov:activity            :a1;    # Cite the influencing Activity.
   ex:foo                   :bar;   # Describe the Activity :a1's influence upon the Entity :e1.
.

:a1 a prov:Activity .
```

The asserter can thus attach additional properties to `:e1Gen` to describe the generation of `:e1` by `:a1`.

As can be seen in this example, qualifying an influence relation provides a second form (e.g. `:e1 prov:qualifiedGeneration :e1Gen`) to express an equivalent influence relation
(e.g. `:e1 prov:wasGeneratedBy :a1`).
It is correct and acceptable for an implementer to use either qualified or unqualified forms as they choose (or both),
and a consuming application should be prepared to recognize either form.
Consuming applications *should* recognize both qualified and unqualified forms, and treat the qualified form as implying the unqualified form.
Because the qualification form is more verbose, the unqualified form should be favored in cases where additional properties are not provided.
When the qualified form is expressed, including the equivalent unqualified form can facilitate PROV-O consumption, and is thus encouraged.

In addition to the previous two tables, [Figure 4](#qualified-terms-figure)
illustrates the classes and properties needed to apply the qualification pattern to ten of the fourteen qualifiable influence relations.
For example, while [prov:qualifiedUsage](#qualifiedUsage),
[prov:Usage](#Usage), and [prov:entity](#p_entity) are used to qualify
prov:used relations,
[prov:qualifiedAssociation](#qualifiedAssociation), [prov:Association](#Association),
and [prov:agent](#p_agent) are used to qualify prov:wasAssociatedWith
relations. This pattern applies to the twelve other influence relations that can be qualified.

In subfigure **a** the prov:qualifiedUsage property parallels the prov:used property and references an instance of
prov:Usage, which in turn provides attributes of the prov:used relation between the Activity and Entity.
The prov:entity property is used to cite the Entity that was used by the Activity.
In this case, the time that the Activity used the Entity is provided using the [prov:atTime](#atTime) property and a literal
xsd:dateTime value. The prov:atTime property can be used to describe any
[prov:InstantaneousEvent](#InstantaneousEvent) (including
[prov:Start](#Start),
[prov:Generation](#Generation),
[prov:Usage](#Usage),
[prov:Invalidation](#Invalidation), and
[prov:End](#End)).

Similarly in subfigure **j**, the prov:qualifiedAssociation property parallels the
prov:wasAssociatedWith property and references an instance of
prov:Association, which in turn provides attributes of the
prov:wasAssociatedWith relation between the Activity and Agent. The
prov:agent property is used to cite the Agent that influenced the Activity.
In this case, the plan of actions and steps that the Agent used to achieve its goals is provided using the
[prov:hadPlan](#hadPlan) property and an instance of [prov:Plan](#Plan). Further, the
[prov:hadRole](#hadRole) property and [prov:Role](#Role)
class can be used to describe the function that the agent served with respect to the Activity.
Both prov:Plan and prov:Role are left to be extended by applications.

![Express association between an activity and an agent using a binary relationship and an alternatie qualified relationship](diagrams/qualified-patterns.svg)

[Figure 4](#qualified-terms-figure):
Illustration of the properties and classes to use (in blue)
to qualify the
[starting point](#description-starting-point-terms) and
[expanded](#description-expanded-terms) influence relations (**dotted black**).   
The diagrams in this document depict Entities as ovals, Activities as rectangles, and Agents as pentagons.
[Quotation](#Quotation), [Revision](#Revision), and [PrimarySource](#PrimarySource)
are omitted because they are special forms of [Derivation](#Derivation) and follow the same pattern as subfigure **g**.

The following two examples show the result of applying the Usage and Association patterns to the chart-making example from [Section 3.1](#description-starting-point-terms).

[Example 8](#narrative-example-qualified-3):

**Qualified Usage**

The prov:qualifiedUsage property parallels the prov:used property to provide an additional description to `:illustrationActivity`. The instance of prov:Usage cites the data used (`:aggregatedByRegions`) and the time the activity used it (`2011-07-14T03:03:03Z`).

```
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.org#> .

:illustrationActivity 
   a prov:Activity;                ## Using Starting Point terms,
   prov:used :aggregatedByRegions; ##   the illustration activity used the aggregated data (to create the bar chart).
.

:aggregatedByRegions a prov:Entity .

:illustrationActivity      
   prov:qualifiedUsage [                                ## Qualify how the :illustrationActivity
      a prov:Usage;                                     ##   used
      prov:entity :aggregatedByRegions;                 ##     the Entity :aggregatedByRegions

      prov:atTime "2011-07-14T03:03:03Z"^^xsd:dateTime; ## Qualification: The aggregated data was used 
                                                        ##   at a particular time to create the bar chart..
   ];
.
```

[Example 9](#narrative-example-qualified-4):

**Qualified Association**

The prov:qualifiedAssociation property parallels the
prov:wasAssociatedWith property to provide an additional description about the `:illustrationActivity` that Derek influenced.
The instance of prov:Association cites the influencing agent (`:derek`) that followed the instructions (`:tutorial_blog`).
Further, Derek served the role of `:illustrationist` during the activity.

```
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.org#> .

:illustrationActivity
   a prov:Activity;                ## Using Starting Point terms,
   prov:wasAssociatedWith :derek;  ##   the illustration activity was associated with Derek in some way.
.

:derek a prov:Agent .

:illustrationActivity
   prov:qualifiedAssociation [       ## Qualify how the :illustrationActivity
      a prov:Association;            ##   was associated with
      prov:agent   :derek            ##     the Agent Derek.

      prov:hadRole :illustrationist; ## Qualification: The role that Derek served.
      prov:hadPlan :tutorial_blog;   ## Qualification: The plan (or recipe, instructions)
                                     ##   that Derek followed when creating the graphical chart.
   ];
.

:tutorial_blog   a prov:Plan, prov:Entity .
:illustrationist a prov:Role .
```

This section finishes with two more examples of qualification as applied to the chart-making example from [Section 3.1](#description-starting-point-terms).

[Example 10](#narrative-example-qualified-5):

**Qualified Generation**

The prov:qualifiedGeneration property parallels the prov:wasGeneratedBy property to provide an additional description to `:bar_chart`. The instance of prov:Generation cites the time (2011-07-14T15:52:14Z) that the activity (`:illustrationActivity`) generated the chart (`:bar_chart`).

```
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.org#> .

:bar_chart
   a prov:Entity;                              ## Using Starting Point terms,
   prov:wasGeneratedBy :illustrationActivity;  ##   the chart was generated in an illustration activity.
.

:illustrationActivity a prov:Activity .

:bar_chart
   prov:qualifiedGeneration [                           ## Qualify how the :bar_chart
      a prov:Generation;                                ##   was generated by
      prov:activity :illustrationActivity;              ##     the Activity :illustrationActivity.

      prov:atTime "2011-07-14T15:52:14Z"^^xsd:dateTime; ## Qualification: The Activity generated
                                                        ##   the bar_chart at a particular time.
   ];
.
```

[Example 11](#narrative-example-qualified-6):

**Qualified Derivation**

The prov:qualifiedDerivation property parallels the prov:wasDerivedFrom
property to provide an additional description to `:bar_chart`. The instance of prov:Derivation
cites the activity (`:illustrationActivity`) and the Usages and Generations that the activity conduced to create the `:bar_chart`.

```
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.org#> .

:bar_chart
   a prov:Entity;                             ## Using Starting Point terms,
   prov:wasDerivedFrom :aggregatedByRegions;  ##   the chart was derived from the aggregated dataset.
.

:aggregatedByRegions a prov:Entity .

:bar_chart
   prov:qualifiedDerivation [                      ## Qualify
      a prov:Derivation;                           ##   how :bar_chart was derived from
      prov:entity        :aggregatedByRegions;     ##     the dataset Entity :aggregatedByRegions.

      prov:hadActivity   :aggregating_activity;    ## Qualification: The activity that derived the :bar_chart.
      prov:hadUsage      :use_of_aggregatedData;   ## Qualification: How the activity used :aggregatedByRegions.
      prov:hadGeneration :generation_of_bar_chart; ## Qualification: How the activity generated the :bar_chart.
   ];
.
```

4. Cross reference for PROV-O classes and properties
----------------------------------------------------

This section provides details for each class and property defined by the PROV Ontology, grouped by the categories described above:

* [Starting Point Terms](#cross-reference-starting-point-terms)
* [Expanded Terms](#cross-reference-expanded-terms)
* [Qualified Terms](#cross-reference-qualified-terms)

The superscripts op and dp denote that a property is an OWL [object property](https://www.w3.org/TR/owl2-syntax/#Object_Properties) or [data property](https://www.w3.org/TR/owl2-syntax/#Data_Properties), respectively.

Each PROV-O term in this cross reference links to the corresponding PROV-DM concept. The PROV-DM's table
[Cross-References to PROV-O and PROV-N](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#prov-dm-to-prov-o-and-prov-n)
provides an overview of the correspondences between PROV-O and PROV-DM.

The qualification classes and properties shown in [Table 2](#qualified-forms-starting-point) and
[Table 3](#qualified-forms-expanded) of the previous section can also be found in each entry of this cross reference.
If the property can be qualified, the **can be qualified with**
header indicates the qualifying property and influence class that should be used.
Conversely, the **qualifies** headers in the listings for qualification terms indicate the unqualified property that they qualify.
In the OWL file itself, the annotation properties prov:qualifiedForm and prov:unqualifiedForm
provide the same linkages between the unqualified properties and their qualifiying terms.

Most examples shown in this cross reference are encoded using the Turtle RDF serialization.
When it is convenient to do so (e.g., when an example describes a prov:Bundle), it may use the [[TRIG](#bib-TRIG)] syntax.
Although this document does not specify how to encode Bundles in RDF, TriG's named graph construct is used only to illustrate the concept of
creating a named set of PROV assertions. Note that [all examples are non-normative](#compliance).

### 4.1 Starting Point Terms

The classes and properties that provide a basis for all other PROV-O terms are discussed in [Section 3.1](#description-starting-point-terms).

* [prov:Entity](#Entity)
* [prov:Activity](#Activity)
* [prov:Agent](#Agent)

* [prov:wasGeneratedBy](#wasGeneratedBy)
* [prov:wasDerivedFrom](#wasDerivedFrom)
* [prov:wasAttributedTo](#wasAttributedTo)
* [prov:startedAtTime](#startedAtTime)
* [prov:used](#used)
* [prov:wasInformedBy](#wasInformedBy)
* [prov:endedAtTime](#endedAtTime)
* [prov:wasAssociatedWith](#wasAssociatedWith)
* [prov:actedOnBehalfOf](#actedOnBehalfOf)

### (1) Class: [prov:Entity](#Entity) back to [starting-point classes](#prov-starting-point-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Entity

An entity is a physical, digital, conceptual, or other kind of thing with some fixed aspects; entities may be real or imaginary.

**Example**

```
@prefix rdfs:    <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix dcterms: <http://purl.org/dc/terms/> .
@prefix prov:    <http://www.w3.org/ns/prov#> .
@prefix :        <http://example.com/> .

:bar_chart 
   a prov:Entity;
   dcterms:title "Aggregated statistics from the crime file"^^xsd:string;
   prov:wasAttributedTo :derek;
.

:derek a prov:Agent .
```

described with properties:

:   [prov:invalidatedAtTime](#invalidatedAtTime "http://www.w3.org/ns/prov#invalidatedAtTime") dp
    , [prov:wasAttributedTo](#wasAttributedTo "http://www.w3.org/ns/prov#wasAttributedTo") op
    , [prov:qualifiedGeneration](#qualifiedGeneration "http://www.w3.org/ns/prov#qualifiedGeneration") op
    , [prov:wasGeneratedBy](#wasGeneratedBy "http://www.w3.org/ns/prov#wasGeneratedBy") op
    , [prov:wasDerivedFrom](#wasDerivedFrom "http://www.w3.org/ns/prov#wasDerivedFrom") op
    , [prov:specializationOf](#specializationOf "http://www.w3.org/ns/prov#specializationOf") op
    , [prov:qualifiedDerivation](#qualifiedDerivation "http://www.w3.org/ns/prov#qualifiedDerivation") op
    , [prov:qualifiedInvalidation](#qualifiedInvalidation "http://www.w3.org/ns/prov#qualifiedInvalidation") op
    , [prov:generatedAtTime](#generatedAtTime "http://www.w3.org/ns/prov#generatedAtTime") dp
    , [prov:qualifiedQuotation](#qualifiedQuotation "http://www.w3.org/ns/prov#qualifiedQuotation") op
    , [prov:hadPrimarySource](#hadPrimarySource "http://www.w3.org/ns/prov#hadPrimarySource") op
    , [prov:qualifiedPrimarySource](#qualifiedPrimarySource "http://www.w3.org/ns/prov#qualifiedPrimarySource") op
    , [prov:alternateOf](#alternateOf "http://www.w3.org/ns/prov#alternateOf") op
    , [prov:value](#value "http://www.w3.org/ns/prov#value") dp
    , [prov:wasInvalidatedBy](#wasInvalidatedBy "http://www.w3.org/ns/prov#wasInvalidatedBy") op
    , [prov:qualifiedAttribution](#qualifiedAttribution "http://www.w3.org/ns/prov#qualifiedAttribution") op
    , [prov:wasQuotedFrom](#wasQuotedFrom "http://www.w3.org/ns/prov#wasQuotedFrom") op
    , [prov:qualifiedRevision](#qualifiedRevision "http://www.w3.org/ns/prov#qualifiedRevision") op
    , [prov:wasRevisionOf](#wasRevisionOf "http://www.w3.org/ns/prov#wasRevisionOf") op
:   [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy") op , [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op , [prov:atLocation](#atLocation "http://www.w3.org/ns/prov#atLocation") op

in range of
:   [prov:hadPrimarySource](#hadPrimarySource "http://www.w3.org/ns/prov#hadPrimarySource") op[prov:generated](#generated "http://www.w3.org/ns/prov#generated") op[prov:wasDerivedFrom](#wasDerivedFrom "http://www.w3.org/ns/prov#wasDerivedFrom") op[prov:entity](#p_entity "http://www.w3.org/ns/prov#entity") op[prov:specializationOf](#specializationOf "http://www.w3.org/ns/prov#specializationOf") op[prov:invalidated](#invalidated "http://www.w3.org/ns/prov#invalidated") op[prov:used](#used "http://www.w3.org/ns/prov#used") op[prov:hadMember](#hadMember "http://www.w3.org/ns/prov#hadMember") op[prov:alternateOf](#alternateOf "http://www.w3.org/ns/prov#alternateOf") op[prov:wasStartedBy](#wasStartedBy "http://www.w3.org/ns/prov#wasStartedBy") op[prov:wasQuotedFrom](#wasQuotedFrom "http://www.w3.org/ns/prov#wasQuotedFrom") op[prov:wasEndedBy](#wasEndedBy "http://www.w3.org/ns/prov#wasEndedBy") op[prov:wasRevisionOf](#wasRevisionOf "http://www.w3.org/ns/prov#wasRevisionOf") op

has subclasses
:   [prov:Collection](#Collection "http://www.w3.org/ns/prov#Collection")
    , [prov:Plan](#Plan "http://www.w3.org/ns/prov#Plan")
    , [prov:Bundle](#Bundle "http://www.w3.org/ns/prov#Bundle")

PROV-DM term
:   [entity](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-entity "prov-dm")

### (2) Class: [prov:Activity](#Activity) back to [starting-point classes](#prov-starting-point-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Activity

An activity is something that occurs over a period of time and acts upon or with entities; it may include consuming, processing, transforming, modifying, relocating, using, or generating entities.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:graduation
   a prov:Activity, :Graduation;
   prov:startedAtTime "2012-04-15T13:00:00-04:00"^^xsd:dateTime;
   prov:used          :ms_smith;
   prov:generated     :doctor_smith;
   prov:endedAtTime   "2012-04-15T14:30:00-04:00"^^xsd:dateTime;
.
:ms_smith     a prov:Entity .
:doctor_smith a prov:Entity .
```

described with properties:

:   [prov:generated](#generated "http://www.w3.org/ns/prov#generated") op
    , [prov:qualifiedAssociation](#qualifiedAssociation "http://www.w3.org/ns/prov#qualifiedAssociation") op
    , [prov:wasAssociatedWith](#wasAssociatedWith "http://www.w3.org/ns/prov#wasAssociatedWith") op
    , [prov:qualifiedEnd](#qualifiedEnd "http://www.w3.org/ns/prov#qualifiedEnd") op
    , [prov:wasEndedBy](#wasEndedBy "http://www.w3.org/ns/prov#wasEndedBy") op
    , [prov:qualifiedUsage](#qualifiedUsage "http://www.w3.org/ns/prov#qualifiedUsage") op
    , [prov:used](#used "http://www.w3.org/ns/prov#used") op
    , [prov:invalidated](#invalidated "http://www.w3.org/ns/prov#invalidated") op
    , [prov:endedAtTime](#endedAtTime "http://www.w3.org/ns/prov#endedAtTime") dp
    , [prov:qualifiedStart](#qualifiedStart "http://www.w3.org/ns/prov#qualifiedStart") op
    , [prov:wasInformedBy](#wasInformedBy "http://www.w3.org/ns/prov#wasInformedBy") op
    , [prov:wasStartedBy](#wasStartedBy "http://www.w3.org/ns/prov#wasStartedBy") op
    , [prov:startedAtTime](#startedAtTime "http://www.w3.org/ns/prov#startedAtTime") dp
    , [prov:qualifiedCommunication](#qualifiedCommunication "http://www.w3.org/ns/prov#qualifiedCommunication") op
:   [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy") op , [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op , [prov:atLocation](#atLocation "http://www.w3.org/ns/prov#atLocation") op

in range of
:   [prov:activity](#p_activity "http://www.w3.org/ns/prov#activity") op[prov:wasInformedBy](#wasInformedBy "http://www.w3.org/ns/prov#wasInformedBy") op[prov:wasGeneratedBy](#wasGeneratedBy "http://www.w3.org/ns/prov#wasGeneratedBy") op[prov:hadActivity](#hadActivity "http://www.w3.org/ns/prov#hadActivity") op[prov:wasInvalidatedBy](#wasInvalidatedBy "http://www.w3.org/ns/prov#wasInvalidatedBy") op

PROV-DM term
:   [Activity](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Activity "prov-dm")

### (3) Class: [prov:Agent](#Agent) back to [starting-point classes](#prov-starting-point-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Agent

An agent is something that bears some form of responsibility for an activity taking place, for the existence of an entity, or for another agent's activity.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix :     <http://example.com#> .

:derek
   a prov:Agent, prov:Person;
   foaf:givenName      "Derek"^^xsd:string;
   foaf:mbox           <mailto:derek@example.org>;
   foaf:homePage       <http://derek.example.com>;
   prov:actedOnBehalfOf :national_newspaper_inc;
.

:national_newspaper_inc
   a prov:Agent, prov:Organization;
   foaf:name "National Newspaper, Inc.";
.
```

described with properties:

:   [prov:actedOnBehalfOf](#actedOnBehalfOf "http://www.w3.org/ns/prov#actedOnBehalfOf") op
    , [prov:qualifiedDelegation](#qualifiedDelegation "http://www.w3.org/ns/prov#qualifiedDelegation") op
:   [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy") op , [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op , [prov:atLocation](#atLocation "http://www.w3.org/ns/prov#atLocation") op

in range of
:   [prov:actedOnBehalfOf](#actedOnBehalfOf "http://www.w3.org/ns/prov#actedOnBehalfOf") op[prov:agent](#p_agent "http://www.w3.org/ns/prov#agent") op[prov:wasAssociatedWith](#wasAssociatedWith "http://www.w3.org/ns/prov#wasAssociatedWith") op[prov:wasAttributedTo](#wasAttributedTo "http://www.w3.org/ns/prov#wasAttributedTo") op

has subclasses
:   [prov:Organization](#Organization "http://www.w3.org/ns/prov#Organization")
    , [prov:Person](#Person "http://www.w3.org/ns/prov#Person")
    , [prov:SoftwareAgent](#SoftwareAgent "http://www.w3.org/ns/prov#SoftwareAgent")

PROV-DM term
:   [agent](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-agent "prov-dm")

### (4) Property: [prov:wasGeneratedBy](#wasGeneratedBy) op back to [starting-point properties](#prov-starting-point-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#wasGeneratedBy

Generation is the completion of production of a new entity by an activity. This entity did not exist before generation and becomes available for usage after this generation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:bar_chart
   a prov:Entity;
   prov:wasGeneratedBy :illustrating;
.

:illustrating a prov:Activity .
```

has super-properties
:   * [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy") op

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

can be qualified with
:   * [prov:Generation](#Generation "http://www.w3.org/ns/prov#Generation")
    * [prov:qualifiedGeneration](#qualifiedGeneration "http://www.w3.org/ns/prov#qualifiedGeneration") op

PROV-DM term
:   [Generation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Generation "prov-dm")

### (5) Property: [prov:wasDerivedFrom](#wasDerivedFrom) op back to [starting-point properties](#prov-starting-point-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#wasDerivedFrom

A derivation is a transformation of an entity into another, an update of an entity resulting in a new one, or the construction of a new entity based on a pre-existing entity.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix ex:   <http://example.com/vocab#> .
@prefix :     <http://example.com/> .

:bar_chart 
   a prov:Entity, ex:Barchart;
   prov:wasDerivedFrom :aggregatedByRegions;
.

:aggregatedByRegions
   a prov:Entity, ex:Dataset;
.
```

The more specific subproperties of prov:wasDerivedFrom (i.e., prov:wasQuotedFrom, prov:wasRevisionOf, prov:hadPrimarySource) should be used when applicable.

has super-properties
:   * [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy") op

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has sub-properties
:   * [prov:hadPrimarySource](#hadPrimarySource "http://www.w3.org/ns/prov#hadPrimarySource")
    * [prov:wasQuotedFrom](#wasQuotedFrom "http://www.w3.org/ns/prov#wasQuotedFrom")
    * [prov:wasRevisionOf](#wasRevisionOf "http://www.w3.org/ns/prov#wasRevisionOf")

can be qualified with
:   * [prov:Derivation](#Derivation "http://www.w3.org/ns/prov#Derivation")
    * [prov:qualifiedDerivation](#qualifiedDerivation "http://www.w3.org/ns/prov#qualifiedDerivation") op

PROV-DM term
:   [Derivation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Derivation "prov-dm")

### (6) Property: [prov:wasAttributedTo](#wasAttributedTo) op back to [starting-point properties](#prov-starting-point-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#wasAttributedTo

Attribution is the ascribing of an entity to an agent.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:geneSequencing
   a prov:Activity;
   prov:startedAtTime "2012-04-25T01:30:00Z"^^xsd:dateTime;
   prov:used              :drosophilaSample-84;
   prov:wasAssociatedWith :lab-technician-GH-32;
   prov:endedAtTime   "2012-04-25T03:40:00Z"^^xsd:dateTime;
.

:drosophilaSample-84  
   a prov:Entity;
   prov:wasAttributedTo :lab-technician-FE-56;
.

:lab-technician-GH-32 a prov:Agent .
:lab-technician-FE-56 a prov:Agent .
```

Attribution is the ascribing of an entity to an agent.

has super-properties
:   * [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy") op

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent")

can be qualified with
:   * [prov:qualifiedAttribution](#qualifiedAttribution "http://www.w3.org/ns/prov#qualifiedAttribution") op
    * [prov:Attribution](#Attribution "http://www.w3.org/ns/prov#Attribution")

PROV-DM term
:   [attribution](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-attribution "prov-dm")

### (7) Property: [prov:startedAtTime](#startedAtTime) dp back to [starting-point properties](#prov-starting-point-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#startedAtTime

Start is when an activity is deemed to have been started by an entity, known as trigger. The activity did not exist before its start. Any usage, generation, or invalidation involving an activity follows the activity's start. A start may refer to a trigger entity that set off the activity, or to an activity, known as starter, that generated the trigger.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:geneSequencing
   a prov:Activity;
   prov:startedAtTime "2012-04-25T01:30:00Z"^^xsd:dateTime;
   prov:used              :drosophilaSample-84;
   prov:wasAssociatedWith :lab-technician-GH-32;
   prov:endedAtTime   "2012-04-25T03:40:00Z"^^xsd:dateTime;
.

:drosophilaSample-84  a prov:Entity .
:lab-technician-GH-32 a prov:Agent .
```

The time at which an activity started. See also prov:endedAtTime.

has domain
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

has range
:   * http://www.w3.org/2001/XMLSchema#dateTime

can be qualified with
:   * [prov:Start](#Start "http://www.w3.org/ns/prov#Start")
    * [prov:atTime](#atTime "http://www.w3.org/ns/prov#atTime") dp

PROV-DM term
:   [Start](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Start "prov-dm")

### (8) Property: [prov:used](#used) op back to [starting-point properties](#prov-starting-point-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#used

Usage is the beginning of utilizing an entity by an activity. Before usage, the activity had not begun to utilize this entity and could not have been affected by the entity.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:sortActivity
   a prov:Activity;
   prov:atTime     "2011-07-16T01:52:02Z"^^xsd:dateTime;
   prov:used       :datasetA;
   prov:generated  :datasetB;
.

:datasetA a prov:Entity.
:datasetB a prov:Entity.

# See qualified Usage for example on how the role of :datasetA can be described for this Activity
```

A prov:Entity that was used by this prov:Activity. For example, :baking prov:used :spoon, :egg, :oven .

has super-properties
:   * [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy") op

has domain
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

has range
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

can be qualified with
:   * [prov:Usage](#Usage "http://www.w3.org/ns/prov#Usage")
    * [prov:qualifiedUsage](#qualifiedUsage "http://www.w3.org/ns/prov#qualifiedUsage") op

PROV-DM term
:   [Usage](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Usage "prov-dm")

### (9) Property: [prov:wasInformedBy](#wasInformedBy) op back to [starting-point properties](#prov-starting-point-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#wasInformedBy

Communication is the exchange of an entity by two activities, one activity using the entity generated by the other.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:writing-celebrity-gossip 
   a prov:Activity;
   prov:wasInformedBy :voicemail-interception;
.

:voicemail-interception a prov:Activity .
```

An activity a2 is dependent on or informed by another activity a1, by way of some unspecified entity that is generated by a1 and used by a2.

has super-properties
:   * [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy") op

has domain
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

has range
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

can be qualified with
:   * [prov:Communication](#Communication "http://www.w3.org/ns/prov#Communication")
    * [prov:qualifiedCommunication](#qualifiedCommunication "http://www.w3.org/ns/prov#qualifiedCommunication") op

PROV-DM term
:   [Communication](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Communication "prov-dm")

### (10) Property: [prov:endedAtTime](#endedAtTime) dp back to [starting-point properties](#prov-starting-point-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#endedAtTime

End is when an activity is deemed to have been ended by an entity, known as trigger. The activity no longer exists after its end. Any usage, generation, or invalidation involving an activity precedes the activity's end. An end may refer to a trigger entity that terminated the activity, or to an activity, known as ender that generated the trigger.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:geneSequencing
   a prov:Activity;
   prov:startedAtTime "2012-04-25T01:30:00Z"^^xsd:dateTime;
   prov:used              :drosophilaSample-84;
   prov:wasAssociatedWith :lab-technician-GH-32;
   prov:endedAtTime   "2012-04-25T03:40:00Z"^^xsd:dateTime;
.

:drosophilaSample-84  a prov:Entity .
:lab-technician-GH-32 a prov:Agent .
```

The time at which an activity ended. See also prov:startedAtTime.

has domain
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

has range
:   * http://www.w3.org/2001/XMLSchema#dateTime

can be qualified with
:   * [prov:End](#End "http://www.w3.org/ns/prov#End")
    * [prov:atTime](#atTime "http://www.w3.org/ns/prov#atTime") dp

PROV-DM term
:   [End](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-End "prov-dm")

### (11) Property: [prov:wasAssociatedWith](#wasAssociatedWith) op back to [starting-point properties](#prov-starting-point-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#wasAssociatedWith

An activity association is an assignment of responsibility to an agent for an activity, indicating that the agent had a role in the activity. It further allows for a plan to be specified, which is the plan intended by the agent to achieve some goals in the context of this activity.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:illustrating
   a prov:Activity; 
   prov:wasAssociatedWith :derek;
.

:derek a prov:Person, prov:Agent, prov:Entity .
```

An prov:Agent that had some (unspecified) responsibility for the occurrence of this prov:Activity.

has super-properties
:   * [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy") op

has domain
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

has range
:   * [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent")

can be qualified with
:   * [prov:Association](#Association "http://www.w3.org/ns/prov#Association")
    * [prov:qualifiedAssociation](#qualifiedAssociation "http://www.w3.org/ns/prov#qualifiedAssociation") op

PROV-DM term
:   [Association](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Association "prov-dm")

### (12) Property: [prov:actedOnBehalfOf](#actedOnBehalfOf) op back to [starting-point properties](#prov-starting-point-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#actedOnBehalfOf

Delegation is the assignment of authority and responsibility to an agent (by itself or by another agent) to carry out a specific activity as a delegate or representative, while the agent it acts on behalf of retains some responsibility for the outcome of the delegated work.
For example, a student acted on behalf of his supervisor, who acted on behalf of the department chair, who acted on behalf of the university; all those agents are responsible in some way for the activity that took place but we do not say explicitly who bears responsibility and to what degree.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:derek
   a prov:Agent;
   foaf:givenName "Derek"^^xsd:string;
   foaf:mbox      <mailto:derek@example.org>;
   prov:actedOnBehalfOf :national_newspaper_inc; 
.

:national_newspaper_inc 
   a prov:Agent, prov:Organization;
   foaf:name "National Newspaper, Inc.";
.
```

An object property to express the accountability of an agent towards another agent. The subordinate agent acted on behalf of the responsible agent in an actual activity.

has super-properties
:   * [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy") op

has domain
:   * [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent")

has range
:   * [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent")

can be qualified with
:   * [prov:Delegation](#Delegation "http://www.w3.org/ns/prov#Delegation")
    * [prov:qualifiedDelegation](#qualifiedDelegation "http://www.w3.org/ns/prov#qualifiedDelegation") op

PROV-DM term
:   [delegation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-delegation "prov-dm")

### 4.2 Expanded Terms

The additional terms used to describe relations among Starting Point classes are discussed in [Section 3.2](#description-expanded-terms).

* [prov:Collection](#Collection)
* [prov:EmptyCollection](#EmptyCollection)
* [prov:Bundle](#Bundle)
* [prov:Person](#Person)
* [prov:SoftwareAgent](#SoftwareAgent)
* [prov:Organization](#Organization)
* [prov:Location](#Location)

* [prov:alternateOf](#alternateOf)
* [prov:specializationOf](#specializationOf)
* [prov:generatedAtTime](#generatedAtTime)
* [prov:hadPrimarySource](#hadPrimarySource)
* [prov:value](#value)
* [prov:wasQuotedFrom](#wasQuotedFrom)
* [prov:wasRevisionOf](#wasRevisionOf)
* [prov:invalidatedAtTime](#invalidatedAtTime)
* [prov:wasInvalidatedBy](#wasInvalidatedBy)
* [prov:hadMember](#hadMember)
* [prov:wasStartedBy](#wasStartedBy)
* [prov:wasEndedBy](#wasEndedBy)
* [prov:invalidated](#invalidated)
* [prov:influenced](#influenced)
* [prov:atLocation](#atLocation)
* [prov:generated](#generated)

### (13) Class: [prov:Collection](#Collection) back to [expanded classes](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Collection

A collection is an entity that provides a structure to some constituents, which are themselves entities. These constituents are said to be member of the collections.

**Example**

```
@prefix rdfs:    <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix dcterms: <http://purl.org/dc/terms/> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix prov:    <http://www.w3.org/ns/prov#> .
@prefix ex:      <http://example.com/ontology#> .
@prefix :        <http://example.com/> .

:todays-us-supreme-court
   a prov:Collection, :RobertsCourt;

   prov:qualifiedGeneration [
      a prov:Generation;
      
      # The generation is being qualified to be imprecise;
      # prov:generatedAtTime and prov:atTime specify exact instants in time.
      dcterms:date "2012"^^xsd:gYear; 
   ];                                 

   prov:hadMember
      <http://dbpedia.org/resource/John_Glover_Roberts,_Jr.>,
      <http://dbpedia.org/resource/Antonin_Scalia>, 
      <http://dbpedia.org/resource/Anthony_Kennedy>, 
      <http://dbpedia.org/resource/Clarence_Thomas>, 
      <http://dbpedia.org/resource/Ruth_Bader_Ginsburg>,
      <http://dbpedia.org/resource/Stephen_Breyer>,     
      <http://dbpedia.org/resource/Samuel_Alito>,      
      <http://dbpedia.org/resource/Sonia_Sotomayor>,  
      <http://dbpedia.org/resource/Elena_Kagan>;   

   prov:wasDerivedFrom :the-first-us-supreme-court;

   dcterms:description :copied-string;
.

:copied-string
   a prov:Entity;
   prov:value """2010–present: A. Scalia A. Kennedy C. Thomas R.B. Ginsburg 
                 S. Breyer S. Alito S. Sotomayor E. Kagan""";
   prov:wasQuotedFrom :page-by-composition;
.

:page-by-seat
   a prov:Entity, ex:WikipediaPage;
   prov:specializationOf <http://purl.org/twc/page/wikipedia/us-supreme-court-by-seat>;
   prov:generatedAtTime "2011-08-31T12:51:00"^^xsd:dateTime;
.

:page-by-composition
   a prov:Entity, ex:WikipediaPage;
   prov:specializationOf <http://purl.org/twc/page/wikipedia/us-supreme-court-by-composition>;
   prov:generatedAtTime "2012-05-16T14:33:00"^^xsd:dateTime;
.
```

is subclass of
:   [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

described with properties:

:   [prov:hadMember](#hadMember "http://www.w3.org/ns/prov#hadMember") op

has subclass
:   [prov:EmptyCollection](#EmptyCollection "http://www.w3.org/ns/prov#EmptyCollection")

PROV-DM term
:   [collection](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-collection "prov-dm")

### (14) Class: [prov:EmptyCollection](#EmptyCollection) back to [expanded classes](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#EmptyCollection

An empty collection is a collection without members.

**Example**

```
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.org/> .

:c a prov:EmptyCollection . # The collection is believed to not contain members.
```

is subclass of
:   [prov:Collection](#Collection "http://www.w3.org/ns/prov#Collection")

described with properties:

:   [prov:hadMember](#hadMember "http://www.w3.org/ns/prov#hadMember") op

### (15) Class: [prov:Bundle](#Bundle) back to [expanded classes](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Bundle

A bundle is a named set of provenance descriptions, and is itself an Entity, so allowing provenance of provenance to be expressed.

**Example**

```
@prefix prov:    <http://www.w3.org/ns/prov#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix my:      <http://example.com/my#> .
@prefix :        <http://example.com/#> .
@base <http://www.example.com/example.ttl> .

<> # A provenance file located at http://www.example.com/example.ttl
   a prov:Bundle;
   prov:generatedAtTime "2012-05-24T09:30:00"^^xsd:dateTime;
   prov:wasAttributedTo :bob;
.

:report1
   a my:Report, prov:Entity;
   my:version "1";
   prov:generatedAtTime "2012-05-24T01:00:00"^^xsd:dateTime;
   prov:wasAttributedTo :bob;
.
```

:   Note that there are kinds of bundles (e.g. handwritten letters, audio recordings, etc.) that are not expressed in PROV-O, but can be still be described by PROV-O.

is subclass of
:   [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

PROV-DM term
:   [bundle-entity](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-bundle-entity "prov-dm")

### (16) Class: [prov:Person](#Person) back to [expanded classes](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Person

Person agents are people.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix :     <http://example.com/> .

<http://dbpedia.org/resource/Pablo_Picasso>
   a prov:Person, prov:Agent;
   foaf:depiction <http://upload.wikimedia.org/wikipedia/commons/9/98/Pablo_picasso_1.jpg>;
.
```

is subclass of
:   [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent")

described with properties:

:   [prov:qualifiedDelegation](#qualifiedDelegation "http://www.w3.org/ns/prov#qualifiedDelegation") op , [prov:actedOnBehalfOf](#actedOnBehalfOf "http://www.w3.org/ns/prov#actedOnBehalfOf") op

PROV-DM term
:   [agent](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-agent "prov-dm")

### (17) Class: [prov:SoftwareAgent](#SoftwareAgent) back to [expanded classes](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#SoftwareAgent

A software agent is running software.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

# Googlebot is Google's web crawling bot; 
# it can initiate and participate in web-crawling activities.

:googlebot
   a prov:SoftwareAgent;
   rdfs:label "Googlebot"^^xsd:string;
.
```

is subclass of
:   [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent")

described with properties:

:   [prov:qualifiedDelegation](#qualifiedDelegation "http://www.w3.org/ns/prov#qualifiedDelegation") op , [prov:actedOnBehalfOf](#actedOnBehalfOf "http://www.w3.org/ns/prov#actedOnBehalfOf") op

PROV-DM term
:   [agent](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-agent "prov-dm")

### (18) Class: [prov:Organization](#Organization) back to [expanded classes](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Organization

An organization is a social or legal institution such as a company, society, etc.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix :     <http://example.com/> .

:W3C 
   a prov:Agent, prov:Organization;
   foaf:name "World Wide Web Consortium";
.
```

is subclass of
:   [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent")

described with properties:

:   [prov:qualifiedDelegation](#qualifiedDelegation "http://www.w3.org/ns/prov#qualifiedDelegation") op , [prov:actedOnBehalfOf](#actedOnBehalfOf "http://www.w3.org/ns/prov#actedOnBehalfOf") op

PROV-DM term
:   [agent](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-agent "prov-dm")

### (19) Class: [prov:Location](#Location) back to [expanded classes](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Location

A location can be an identifiable geographic place (ISO 19112), but it can also be a non-geographic place such as a directory, row, or column. As such, there are numerous ways in which location can be expressed, such as by a coordinate, address, landmark, and so forth.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix sioc: <http://rdfs.org/sioc/ns#> .
@prefix :     <http://example.com/> .

# A Location can be a path or a geographical location.

:post9821 
   a prov:Entity, sioc:Post;   
   prov:wasGeneratedBy :publicationActivity1123;
   prov:atLocation     :more-crime-happens-in-cities;
   prov:qualifiedGeneration [
      a prov:Generation;
      prov:activity    :publicationActivity1123;
      prov:atTime     "2011-07-16T01:52:02Z"^^xsd:dateTime; 
      prov:atLocation <http://dbpedia.org/resource/Madrid>;
   ];
.

:publicationActivity1123      a prov:Activity.
:more-crime-happens-in-cities a prov:Location.
<http://dbpedia.org/resource/Madrid> a prov:Location.
```

in range of
:   [prov:atLocation](#atLocation "http://www.w3.org/ns/prov#atLocation") op

PROV-DM term
:   [attribute-location](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-attribute-location "prov-dm")

### (20) Property: [prov:alternateOf](#alternateOf) op back to [expanded properties](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#alternateOf

Two alternate entities present aspects of the same thing. These aspects may be the same or different, and the alternate entities may or may not overlap in time.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:bbc a prov:Agent .

:london_forecast_0412 
   a prov:Entity;
   prov:wasAttributedTo :bbc;
   prov:wasGeneratedBy [
      a prov:Activity;
      prov:endedAtTime "2012-04-12T00:00:00-04:00"^^xsd:dateTime;
   ];
   prov:alternateOf :london_forecast_0413;
.

:london_forecast_0413 
   a prov:Entity;
   prov:wasAttributedTo :bbc;
   prov:wasGeneratedBy [
      a prov:Activity;
      prov:endedAtTime "2012-04-13T00:00:00-04:00"^^xsd:dateTime;
   ];
   prov:alternateOf :london_forecast_0412;
.

## :london_forecast_0412 and :london_forecast_0413 are both 
## specialization of the more general entity :london_forecast

:london_forecast 
   a prov:Entity;
   prov:wasAttributedTo :bbc;
.

:london_forecast_0412
   prov:specializationOf :london_forecast;
.
:london_forecast_0413
   prov:specializationOf :london_forecast;
.
```

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has sub-properties
:   * [prov:specializationOf](#specializationOf "http://www.w3.org/ns/prov#specializationOf")

PROV-DM term
:   [alternate](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-alternate "prov-dm")

### (21) Property: [prov:specializationOf](#specializationOf) op back to [expanded properties](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#specializationOf

An entity that is a specialization of another shares all aspects of the latter, and additionally presents more specific aspects of the same thing as the latter. In particular, the lifetime of the entity being specialized contains that of any specialization. Examples of aspects include a time period, an abstraction, and a context associated with the entity.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:london_forecast_0412 
   a prov:Entity;
   prov:wasAttributedTo :bbc;
   prov:wasGeneratedBy [
      a prov:Activity;
      prov:endedAtTime "2012-04-12T00:00:00-04:00"^^xsd:dateTime;
   ];
.

:london_forecast_0413 
   a prov:Entity;
   prov:wasAttributedTo :bbc;
   prov:wasGeneratedBy [
      a prov:Activity;
      prov:endedAtTime "2012-04-13T00:00:00-04:00"^^xsd:dateTime;
   ];
.

:london_forecast 
   a prov:Entity;
   prov:wasAttributedTo :bbc;
.

## :london_forecast_0412 and :london_forecast_0413 are both 
## specialization of the more general entity :london_forecast

:london_forecast_0412  
   prov:alternateOf      :london_forecast_0413;
   prov:specializationOf :london_forecast;
.
```

has super-properties
:   * [prov:alternateOf](#alternateOf "http://www.w3.org/ns/prov#alternateOf") op

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

PROV-DM term
:   [specialization](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-specialization "prov-dm")

### (22) Property: [prov:generatedAtTime](#generatedAtTime) dp back to [expanded properties](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#generatedAtTime

Generation is the completion of production of a new entity by an activity. This entity did not exist before generation and becomes available for usage after this generation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

# A widget was generated 1:35:23 PM on April 3, 2012 UTC

:widget-789532
   a prov:Entity;
   prov:generatedAtTime "2012-04-03T13:35:23Z"^^xsd:dateTime;
.

# The above statement is equivalent to:
# :widget-789532 prov:qualifiedGeneration [ prov:atTime "2012-04-03T13:35:23Z"^^xsd:dateTime ] .
```

The time at which an entity was completely created and is available for use.

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * http://www.w3.org/2001/XMLSchema#dateTime

can be qualified with
:   * [prov:Generation](#Generation "http://www.w3.org/ns/prov#Generation")
    * [prov:atTime](#atTime "http://www.w3.org/ns/prov#atTime") dp

PROV-DM term
:   [Generation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Generation "prov-dm")

### (23) Property: [prov:hadPrimarySource](#hadPrimarySource) op back to [expanded properties](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#hadPrimarySource

A primary source for a topic refers to something produced by some agent with direct experience and knowledge about the topic, at the time of the topic's study, without benefit from hindsight.
Because of the directness of primary sources, they 'speak for themselves' in ways that cannot be captured through the filter of secondary sources. As such, it is important for secondary sources to reference those primary sources from which they were derived, so that their reliability can be investigated.
A primary source relation is a particular case of derivation of secondary materials from their primary sources. It is recognized that the determination of primary sources can be up to interpretation, and should be done according to conventions accepted within the application's domain.

**Example**

```
@prefix rdfs:    <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix foaf:    <http://xmlns.com/foaf/0.1/> .
@prefix prov:    <http://www.w3.org/ns/prov#> .
@prefix lang:    <http://lexvo.org/id/iso639-3/> .
@prefix dcterms: <http://purl.org/dc/terms/> .
@prefix frbr:    <http://purl.org/vocab/frbr/core#> .
@prefix :        <http://example.com/> .

## Having an primary source is a particular case of derivation.

<http://www.gutenberg.org/ebooks/996>
   a prov:Entity, frbr:Work;
   dcterms:title          "Don Quixote";
   prov:wasAttributedTo   :ormsby;
   dcterms:language       lang:eng;
   prov:hadPrimarySource <http://cultura.linkeddata.es/BNE/resource/C1001/XX2197892>;
.

#### The English version book is a translation that is based on the original Spanish book

<http://cultura.linkeddata.es/BNE/resource/C1001/XX2197892>
    a prov:Entity, frbr:Work;
    prov:wasAttributedTo :cervantes;
    dcterms:language     lang:spa;
.

:cervantes
   a prov:Person;
   foaf:name "Miguel de Cervantes";
.

:ormsby
   a prov:Person;
   foaf:name "John Ormsby";
.
```

has super-properties
:   * [prov:wasDerivedFrom](#wasDerivedFrom "http://www.w3.org/ns/prov#wasDerivedFrom") op

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

can be qualified with
:   * [prov:qualifiedPrimarySource](#qualifiedPrimarySource "http://www.w3.org/ns/prov#qualifiedPrimarySource") op
    * [prov:PrimarySource](#PrimarySource "http://www.w3.org/ns/prov#PrimarySource")

PROV-DM term
:   [primary-source](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-primary-source "prov-dm")

### (24) Property: [prov:value](#value) dp back to [expanded properties](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#value

Provides a value that is a direct representation of an entity.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:copied-string
   a prov:Entity;
   prov:value 
      """2010–present: A. Scalia A. Kennedy C. Thomas R.B. Ginsburg 
         S. Breyer S. Alito S. Sotomayor E. Kagan""";
   prov:wasQuotedFrom 
      <http://purl.org/twc/page/wikipedia/us-supreme-court-by-composition>;
.
```

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

PROV-DM term
:   [attribute-value](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-attribute-value "prov-dm")

### (25) Property: [prov:wasQuotedFrom](#wasQuotedFrom) op back to [expanded properties](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#wasQuotedFrom

A quotation is the repeat of (some or all of) an entity, such as text or image, by someone who may or may not be its original author. Quotation is a particular case of derivation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:bl-dagstuhl
   a prov:Entity;

   prov:value """During the workshop, it became clear to me that the consensus
   based models (which are often graphical in nature) can not only be
   formalized but also be directly connected to these database focused
   formalizations. I just needed to get over the differences in syntax.
   This could imply that we could have nice way to trace provenance across
   systems and through databases and be able to understand the
   mathematical properties of this interconnection.""";

   prov:wasQuotedFrom <http://purl.org/twc/page/thoughts-from-the-dagstuhl-workshop>;
.

<http://purl.org/twc/page/thoughts-from-the-dagstuhl-workshop>
   a prov:Entity;
.
```

An entity is derived from an original entity by copying, or 'quoting', some or all of it.

has super-properties
:   * [prov:wasDerivedFrom](#wasDerivedFrom "http://www.w3.org/ns/prov#wasDerivedFrom") op

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

can be qualified with
:   * [prov:qualifiedQuotation](#qualifiedQuotation "http://www.w3.org/ns/prov#qualifiedQuotation") op
    * [prov:Quotation](#Quotation "http://www.w3.org/ns/prov#Quotation")

PROV-DM term
:   [quotation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-quotation "prov-dm")

### (26) Property: [prov:wasRevisionOf](#wasRevisionOf) op back to [expanded properties](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#wasRevisionOf

A revision is a derivation for which the resulting entity is a revised version of some original. The implication here is that the resulting entity contains substantial content from the original. Revision is a particular case of derivation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix sioc: <http://rdfs.org/sioc/ns#> .
@prefix :     <http://example.com/> .

:post9821v1
   a prov:Entity, sioc:Post;
   prov:wasRevisionOf :post9821;
   rdfs:comment ":post9821v1 is a post, which is a revision of the original post :post9821.";
.
```

A revision is a derivation that revises an entity into a revised version.

has super-properties
:   * [prov:wasDerivedFrom](#wasDerivedFrom "http://www.w3.org/ns/prov#wasDerivedFrom") op

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

can be qualified with
:   * [prov:Revision](#Revision "http://www.w3.org/ns/prov#Revision")
    * [prov:qualifiedRevision](#qualifiedRevision "http://www.w3.org/ns/prov#qualifiedRevision") op

PROV-DM term
:   [revision](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-revision "prov-dm")

### (27) Property: [prov:invalidatedAtTime](#invalidatedAtTime) dp back to [expanded properties](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#invalidatedAtTime

Invalidation is the start of the destruction, cessation, or expiry of an existing entity by an activity. The entity is no longer available for use (or further invalidation) after invalidation. Any generation or usage of an entity precedes its invalidation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix ex:   <http://example.com/ontology#> .
@prefix :     <http://example.com/> .

:the-Painter 
   a prov:Entity, ex:Painting;
   rdfs:label "Le Peintre"@fr, "The Painter"@en;
   prov:wasAttributedTo <http://dbpedia.org/resource/Pablo_Picasso>;
   prov:invalidatedAtTime "1998-09-02T01:31:00Z"^^xsd:dateTime;
.
```

The time at which an entity was invalidated (i.e., no longer usable).

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * http://www.w3.org/2001/XMLSchema#dateTime

can be qualified with
:   * [prov:Invalidation](#Invalidation "http://www.w3.org/ns/prov#Invalidation")
    * [prov:atTime](#atTime "http://www.w3.org/ns/prov#atTime") dp

PROV-DM term
:   [Invalidation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Invalidation "prov-dm")

### (28) Property: [prov:wasInvalidatedBy](#wasInvalidatedBy) op back to [expanded properties](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#wasInvalidatedBy

Invalidation is the start of the destruction, cessation, or expiry of an existing entity by an activity. The entity is no longer available for use (or further invalidation) after invalidation. Any generation or usage of an entity precedes its invalidation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix :     <http://example.com/> .

:the-Painter 
   a prov:Entity, :Painting;
   rdfs:label "Le Peintre"@fr, "The Painter"@en;
   prov:wasAttributedTo <http://dbpedia.org/resource/Pablo_Picasso>;
   prov:wasInvalidatedBy :Swissair_Flight_111_crash; #The painting was destroyed in an airplane crash
.

<http://dbpedia.org/resource/Pablo_Picasso> 
   a prov:Agent;
   foaf:depiction <http://upload.wikimedia.org/wikipedia/commons/9/98/Pablo_picasso_1.jpg>;
.

:Swissair_Flight_111_crash 
   a prov:Activity;
   prov:used <http://dbpedia.org/resource/Swissair_Flight_111>;
.
```

has super-properties
:   * [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy") op

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

can be qualified with
:   * [prov:Invalidation](#Invalidation "http://www.w3.org/ns/prov#Invalidation")
    * [prov:qualifiedInvalidation](#qualifiedInvalidation "http://www.w3.org/ns/prov#qualifiedInvalidation") op

PROV-DM term
:   [Invalidation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Invalidation "prov-dm")

### (29) Property: [prov:hadMember](#hadMember) op back to [expanded properties](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#hadMember

A collection is an entity that provides a structure to some constituents, which are themselves entities. These constituents are said to be member of the collections.

**Example**

```
@prefix rdfs:    <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix dcterms: <http://purl.org/dc/terms/> .
@prefix prov:    <http://www.w3.org/ns/prov#> .
@prefix ex:      <http://example.com/ontology#> .
@prefix :        <http://example.com/> .

:todays-us-supreme-court
   a prov:Collection, ex:RobertsCourt;
   dcterms:description [
      a prov:Entity;
      prov:value """2010–present: A. Scalia A. Kennedy C. Thomas R.B. Ginsburg S. 
                  Breyer S. Alito S. Sotomayor E. Kagan""";
      prov:wasQuotedFrom :page-by-composition;
   ];
   prov:qualifiedGeneration [
      a prov:Generation;
      
      # Since we need to be imprecise, we can't use prov:generatedAtTime or prov:atTime
      dcterms:date "2012"^^xsd:gYear;
   ];

   prov:wasDerivedFrom :the-first-us-supreme-court;

   prov:hadMember
      <http://dbpedia.org/resource/John_Glover_Roberts,_Jr.>,
      <http://dbpedia.org/resource/Antonin_Scalia>, 
      <http://dbpedia.org/resource/Anthony_Kennedy>, 
      <http://dbpedia.org/resource/Clarence_Thomas>, 
      <http://dbpedia.org/resource/Ruth_Bader_Ginsburg>,
      <http://dbpedia.org/resource/Stephen_Breyer>,     
      <http://dbpedia.org/resource/Samuel_Alito>,      
      <http://dbpedia.org/resource/Sonia_Sotomayor>,  
      <http://dbpedia.org/resource/Elena_Kagan>;   
.

:page-by-seat
   a prov:Entity, ex:WikipediaPage;
   prov:specializationOf <http://purl.org/twc/page/wikipedia/us-supreme-court-by-seat>;
   prov:generatedAtTime "2011-08-31T12:51:00"^^xsd:dateTime;
.

:page-by-composition
   a prov:Entity, ex:WikipediaPage;
   prov:specializationOf <http://purl.org/twc/page/wikipedia/us-supreme-court-by-composition>;
   prov:generatedAtTime "2012-05-16T14:33:00"^^xsd:dateTime;
.
```

has super-properties
:   * [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy") op

has domain
:   * [prov:Collection](#Collection "http://www.w3.org/ns/prov#Collection")

has range
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

PROV-DM term
:   [collection](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-collection "prov-dm")

### (30) Property: [prov:wasStartedBy](#wasStartedBy) op back to [expanded properties](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#wasStartedBy

Start is when an activity is deemed to have been started by an entity, known as trigger. The activity did not exist before its start. Any usage, generation, or invalidation involving an activity follows the activity's start. A start may refer to a trigger entity that set off the activity, or to an activity, known as starter, that generated the trigger.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

# Use prov:qualifiedStart to see when and where the activity was started

:experiment
   a prov:Activity;
   prov:wasStartedBy :researcher;
.

:researcher a prov:Agent .
```

Start is when an activity is deemed to have started. A start may refer to an entity, known as trigger, that initiated the activity.

has super-properties
:   * [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy") op

has domain
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

has range
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

can be qualified with
:   * [prov:Start](#Start "http://www.w3.org/ns/prov#Start")
    * [prov:qualifiedStart](#qualifiedStart "http://www.w3.org/ns/prov#qualifiedStart") op

PROV-DM term
:   [Start](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Start "prov-dm")

### (31) Property: [prov:wasEndedBy](#wasEndedBy) op back to [expanded properties](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#wasEndedBy

End is when an activity is deemed to have been ended by an entity, known as trigger. The activity no longer exists after its end. Any usage, generation, or invalidation involving an activity precedes the activity's end. An end may refer to a trigger entity that terminated the activity, or to an activity, known as ender that generated the trigger.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:experiment 
   a prov:Activity;
   prov:wasEndedBy :inconsistentResult;
   prov:qualifiedEnd [
      a prov:End;
      prov:entity     :inconsistentResult;
      prov:atTime    "2011-07-16T01:52:02Z"^^xsd:dateTime;
      prov:atLocation :scienceLab_003;
   ];
.

:inconsistentResult a prov:Entity .
:scienceLab_003     a prov:Location .
```

End is when an activity is deemed to have ended. An end may refer to an entity, known as trigger, that terminated the activity.

has super-properties
:   * [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy") op

has domain
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

has range
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

can be qualified with
:   * [prov:End](#End "http://www.w3.org/ns/prov#End")
    * [prov:qualifiedEnd](#qualifiedEnd "http://www.w3.org/ns/prov#qualifiedEnd") op

PROV-DM term
:   [End](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-End "prov-dm")

### (32) Property: [prov:invalidated](#invalidated) op back to [expanded properties](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#invalidated

Invalidation is the start of the destruction, cessation, or expiry of an existing entity by an activity. The entity is no longer available for use (or further invalidation) after invalidation. Any generation or usage of an entity precedes its invalidation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix ex:   <http://example.com/ontology#> .
@prefix :     <http://example.com/> .

:swissair_Flight_111_crash 
   a prov:Activity;
   prov:used        <http://dbpedia.org/resource/Swissair_Flight_111>;
   prov:invalidated :the-Painter;
.

:the-Painter 
   a prov:Entity, ex:Painting;
   rdfs:label "Le Peintre"@fr, "The Painter"@en;
   prov:wasAttributedTo <http://dbpedia.org/resource/Pablo_Picasso>;
   
   # Inferred from prov:invalidated
   prov:wasInvalidatedBy :swissair_Flight_111_crash;   
.

<http://dbpedia.org/resource/Pablo_Picasso> 
   a prov:Agent;
   foaf:depiction <http://upload.wikimedia.org/wikipedia/commons/9/98/Pablo_picasso_1.jpg>;
.
```

has super-properties
:   * [prov:influenced](#influenced "http://www.w3.org/ns/prov#influenced") op

has domain
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

has range
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has inverse
:   * [prov:wasInvalidatedBy](#wasInvalidatedBy "http://www.w3.org/ns/prov#wasInvalidatedBy")

PROV-DM term
:   [Invalidation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Invalidation "prov-dm")

### (33) Property: [prov:influenced](#influenced) op back to [expanded properties](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#influenced

Influence is the capacity of an entity, activity, or agent to have an effect on the character, development, or behavior of another by means of usage, start, end, generation, invalidation, communication, derivation, attribution, association, or delegation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix w3:   <http://example.com/w3/> .
@prefix tr:   <http://example.com/tech-report/> .
@prefix :     <http://example.com/> .

# prov:influenced is a top-level property that links any
# Entity, Activity, or Agent to any other 
# Entity, Activity, or Agent that it had an effect upon.

w3:Consortium 
   a prov:Agent;
   prov:influenced tr:WD-prov-dm-20111215;
.
```

has inverse
:   * [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy")

has sub-properties
:   * [prov:generated](#generated "http://www.w3.org/ns/prov#generated")
    * [prov:invalidated](#invalidated "http://www.w3.org/ns/prov#invalidated")

PROV-DM term
:   [influence](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-influence "prov-dm")

### (34) Property: [prov:atLocation](#atLocation) op back to [expanded properties](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#atLocation

A location can be an identifiable geographic place (ISO 19112), but it can also be a non-geographic place such as a directory, row, or column. As such, there are numerous ways in which location can be expressed, such as by a coordinate, address, landmark, and so forth.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix sioc: <http://rdfs.org/sioc/ns#> .
@prefix :     <http://example.com/> .

# A Location can be a path or a geographical location.

:post9821 
   a prov:Entity, sioc:Post;   
   prov:wasGeneratedBy :publicationActivity1123;
   prov:atLocation     :more-crime-happens-in-cities;
   prov:qualifiedGeneration [
      a prov:Generation;
      prov:activity    :publicationActivity1123;
      prov:atTime     "2011-07-16T01:52:02Z"^^xsd:dateTime; 
      prov:atLocation <http://dbpedia.org/resource/Madrid>;
   ];
.

:publicationActivity1123      a prov:Activity .
:more-crime-happens-in-cities        a prov:Location .
<http://dbpedia.org/resource/Madrid> a prov:Location .
```

The Location of any resource.

This property has multiple RDFS domains to suit multiple OWL Profiles. See [PROV-O OWL Profile](#owl-profile).

has domain
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")
      or [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent")
      or [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")
      or [prov:InstantaneousEvent](#InstantaneousEvent "http://www.w3.org/ns/prov#InstantaneousEvent")

has range
:   * [prov:Location](#Location "http://www.w3.org/ns/prov#Location")

PROV-DM term
:   [attribute-location](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-attribute-location "prov-dm")

### (35) Property: [prov:generated](#generated) op back to [expanded properties](#prov-expanded-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#generated

Generation is the completion of production of a new entity by an activity. This entity did not exist before generation and becomes available for usage after this generation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:proteinDigestion
   a prov:Activity;
   prov:generated :peptideSample1;
.

:peptideSample1 a prov:Entity .
```

has super-properties
:   * [prov:influenced](#influenced "http://www.w3.org/ns/prov#influenced") op

has domain
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

has range
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has inverse
:   * [prov:wasGeneratedBy](#wasGeneratedBy "http://www.w3.org/ns/prov#wasGeneratedBy")

PROV-DM term
:   [Generation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Generation "prov-dm")

### 4.3 Qualified Terms

The terms used to qualify the Starting Point and Expanded properties are discussed in [Section 3.3](#description-qualified-terms).

* [prov:Influence](#Influence)
* [prov:EntityInfluence](#EntityInfluence)
* [prov:Usage](#Usage)
* [prov:Start](#Start)
* [prov:End](#End)
* [prov:Derivation](#Derivation)
* [prov:PrimarySource](#PrimarySource)
* [prov:Quotation](#Quotation)
* [prov:Revision](#Revision)
* [prov:ActivityInfluence](#ActivityInfluence)
* [prov:Generation](#Generation)
* [prov:Communication](#Communication)
* [prov:Invalidation](#Invalidation)
* [prov:AgentInfluence](#AgentInfluence)
* [prov:Attribution](#Attribution)
* [prov:Association](#Association)
* [prov:Plan](#Plan)
* [prov:Delegation](#Delegation)
* [prov:InstantaneousEvent](#InstantaneousEvent)
* [prov:Role](#Role)

* [prov:wasInfluencedBy](#wasInfluencedBy)
* [prov:qualifiedInfluence](#qualifiedInfluence)
* [prov:qualifiedGeneration](#qualifiedGeneration)
* [prov:qualifiedDerivation](#qualifiedDerivation)
* [prov:qualifiedPrimarySource](#qualifiedPrimarySource)
* [prov:qualifiedQuotation](#qualifiedQuotation)
* [prov:qualifiedRevision](#qualifiedRevision)
* [prov:qualifiedAttribution](#qualifiedAttribution)
* [prov:qualifiedInvalidation](#qualifiedInvalidation)
* [prov:qualifiedStart](#qualifiedStart)
* [prov:qualifiedUsage](#qualifiedUsage)
* [prov:qualifiedCommunication](#qualifiedCommunication)
* [prov:qualifiedAssociation](#qualifiedAssociation)
* [prov:qualifiedEnd](#qualifiedEnd)
* [prov:qualifiedDelegation](#qualifiedDelegation)
* [prov:influencer](#influencer)
* [prov:entity](#p_entity)
* [prov:hadUsage](#hadUsage)
* [prov:hadGeneration](#hadGeneration)
* [prov:activity](#p_activity)
* [prov:agent](#p_agent)
* [prov:hadPlan](#hadPlan)
* [prov:hadActivity](#hadActivity)
* [prov:atTime](#atTime)
* [prov:hadRole](#hadRole)

### (36) Class: [prov:Influence](#Influence) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Influence

Influence is the capacity of an entity, activity, or agent to have an effect on the character, development, or behavior of another by means of usage, start, end, generation, invalidation, communication, derivation, attribution, association, or delegation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix my:   <http://example.com/ontology#> .
@prefix :     <http://example.com/> .

# Although a domain extension (e.g. ':wasConductedBy') is not defined by PROV-O,
# the relation between a surgery and an agent can still be qualified
# by reusing prov:Influence and one of its three subclasses 
# (depending on the type of influencer):
# AgentInfluence, EntityInfluence, and ActivityInfluence.

my:wasConductedBy rdfs:subPropertyOf prov:wasAssociatedWith .

:conductingSurgery_1
   a prov:Activity;
   
   # This unqualified influence is unknown in PROV, 
   # but would be a subproperty of wasAssociatedWith.
   my:wasConductedBy :bob;     
   
   # Even though PROV systems do not understand my:wasConductedBy, 
   prov:qualifiedAssociation [ 
      # they can recognize that the unknown relation 
      # is being qualified with a prov:hadRole.
      a prov:Association,     
        prov:AgentInfluence,   # Inferred
        prov:Influence;        # Inferred
      prov:agent   :bob;       # The object of my:wasConductedBy
      prov:hadRole my:surgeon;
   ];
.

:bob       a prov:Agent .
my:surgeon a prov:Role .
```

:   Because prov:Influence is a broad relation, its most specific subclasses (e.g. prov:Communication, prov:Delegation, prov:End, prov:Revision, etc.) should be used when applicable.
:   An instance of prov:Influence provides additional descriptions about the binary prov:wasInfluencedBy relation from some influenced Activity, Entity, or Agent to the influencing Activity, Entity, or Agent. For example, :stomach\_ache prov:wasInfluencedBy :spoon; prov:qualifiedInfluence [ a prov:Influence; prov:entity :spoon; :foo :bar ] . Because prov:Influence is a broad relation, the more specific relations (Communication, Delegation, End, etc.) should be used when applicable.

described with properties:

:   [prov:influencer](#influencer "http://www.w3.org/ns/prov#influencer") op
    , [prov:hadRole](#hadRole "http://www.w3.org/ns/prov#hadRole") op
    , [prov:hadActivity](#hadActivity "http://www.w3.org/ns/prov#hadActivity") op

in range of
:   [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op

has subclasses
:   [prov:ActivityInfluence](#ActivityInfluence "http://www.w3.org/ns/prov#ActivityInfluence")
    , [prov:AgentInfluence](#AgentInfluence "http://www.w3.org/ns/prov#AgentInfluence")
    , [prov:EntityInfluence](#EntityInfluence "http://www.w3.org/ns/prov#EntityInfluence")

qualifies
:   [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy") op

PROV-DM term
:   [influence](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-influence "prov-dm")

### (37) Class: [prov:EntityInfluence](#EntityInfluence) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#EntityInfluence

EntityInfluence is the capacity of an entity to have an effect on the character, development, or behavior of another by means of usage, start, end, derivation, or other.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:sortActivity
   a prov:Activity;
   prov:used :rawData;
   prov:qualifiedUsage [
      a prov:Usage, 
        prov:EntityInfluence; ## Instances of Start, End, Usage, Derivation, and Invalidation
      prov:entity  :datasetA; ## qualify the influenced of an Entity (cited by prov:entity).
      prov:hadRole :inputToBeSorted;
   ];
   prov:generated :sortedData;
.

:rawData    a prov:Entity .
:sortedData a prov:Entity .
```

:   EntityInfluence provides additional descriptions of an Entity's binary influence upon any other kind of resource. Instances of EntityInfluence use the prov:entity property to cite the influencing Entity.
:   It is not recommended that the type EntityInfluence be asserted without also asserting one of its more specific subclasses.

is subclass of
:   [prov:Influence](#Influence "http://www.w3.org/ns/prov#Influence")

described with properties:

:   [prov:entity](#p_entity "http://www.w3.org/ns/prov#entity") op
:   [prov:hadRole](#hadRole "http://www.w3.org/ns/prov#hadRole") op , [prov:influencer](#influencer "http://www.w3.org/ns/prov#influencer") op , [prov:hadActivity](#hadActivity "http://www.w3.org/ns/prov#hadActivity") op

has subclasses
:   [prov:End](#End "http://www.w3.org/ns/prov#End")
    , [prov:Start](#Start "http://www.w3.org/ns/prov#Start")
    , [prov:Usage](#Usage "http://www.w3.org/ns/prov#Usage")
    , [prov:Derivation](#Derivation "http://www.w3.org/ns/prov#Derivation")

### (38) Class: [prov:Usage](#Usage) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Usage

Usage is the beginning of utilizing an entity by an activity. Before usage, the activity had not begun to utilize this entity and could not have been affected by the entity.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:sortActivity 
   a prov:Activity;
   prov:startedAtTime  "2011-07-16T01:52:02Z"^^xsd:dateTime; 
   prov:qualifiedUsage [
      a prov:Usage;
      prov:entity    :datasetA;         ## The entity used by the prov:Usage
      prov:hadRole   :inputToBeSorted;  ## the role of the entity in this prov:Usage        
   ];
   prov:generated :datasetB;
.

:datasetA        a prov:Entity .
:datasetB        a prov:Entity .
:inputToBeSorted a prov:Role .

## The role of :datasetA cannot be expressed using only starting-point terms:

:sortActivity
   a prov:Activity;
   prov:startedAtTime     "2011-07-16T01:52:02Z"^^xsd:dateTime;
   prov:used       :datasetA;
   prov:generated  :datasetB;
.
```

:   An instance of prov:Usage provides additional descriptions about the binary prov:used relation from some prov:Activity to an prov:Entity that it used. For example, :keynote prov:used :podium; prov:qualifiedUsage [ a prov:Usage; prov:entity :podium; :foo :bar ].

is subclass of
:   [prov:InstantaneousEvent](#InstantaneousEvent "http://www.w3.org/ns/prov#InstantaneousEvent")
    , [prov:EntityInfluence](#EntityInfluence "http://www.w3.org/ns/prov#EntityInfluence")

described with properties:

:   [prov:atTime](#atTime "http://www.w3.org/ns/prov#atTime") dp , [prov:entity](#p_entity "http://www.w3.org/ns/prov#entity") op

in range of
:   [prov:hadUsage](#hadUsage "http://www.w3.org/ns/prov#hadUsage") op[prov:qualifiedUsage](#qualifiedUsage "http://www.w3.org/ns/prov#qualifiedUsage") op

qualifies
:   [prov:used](#used "http://www.w3.org/ns/prov#used") op

PROV-DM term
:   [Usage](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Usage "prov-dm")

### (39) Class: [prov:Start](#Start) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Start

Start is when an activity is deemed to have been started by an entity, known as trigger. The activity did not exist before its start. Any usage, generation, or invalidation involving an activity follows the activity's start. A start may refer to a trigger entity that set off the activity, or to an activity, known as starter, that generated the trigger.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

### Start can be used to qualify wasStartedBy with time and location information.
### In this example, a consistency checking activity is started by the update of a data record.

:consistency_checking
   a prov:Activity;
   prov:wasStartedBy :updated_data_record;
   prov:qualifiedStart [
      a prov:Start;
      prov:entity       :updated_data_record;
      prov:atTime       "2011-07-06T01:48:36Z"^^xsd:dateTime;
      prov:atLocation   :scienceLab_003;
      prov:hadActivity  :syntax_checking;
   ];
.

:updated_data_record a prov:Entity .

### There is an explicit process of checking the syntax of the updated data record

:syntax_checking
   a   prov:Activity ;
   prov:startedAtTime      "2011-07-06T01:48:36Z"^^xsd:dateTime;
   prov:endedAtTime        "2011-07-06T02:12:36Z"^^xsd:dateTime;
   prov:wasAssociatedWith  :syntax_checker ;
.

:syntax_checker   a   prov:SoftwareAgent .
```

:   An instance of prov:Start provides additional descriptions about the binary prov:wasStartedBy relation from some started prov:Activity to an prov:Entity that started it. For example, :foot\_race prov:wasStartedBy :bang; prov:qualifiedStart [ a prov:Start; prov:entity :bang; :foo :bar; prov:atTime '2012-03-09T08:05:08-05:00'^^xsd:dateTime ] .

is subclass of
:   [prov:InstantaneousEvent](#InstantaneousEvent "http://www.w3.org/ns/prov#InstantaneousEvent")
    , [prov:EntityInfluence](#EntityInfluence "http://www.w3.org/ns/prov#EntityInfluence")

described with properties:

:   [prov:hadActivity](#hadActivity "http://www.w3.org/ns/prov#hadActivity") op
:   [prov:atTime](#atTime "http://www.w3.org/ns/prov#atTime") dp , [prov:entity](#p_entity "http://www.w3.org/ns/prov#entity") op

in range of
:   [prov:qualifiedStart](#qualifiedStart "http://www.w3.org/ns/prov#qualifiedStart") op

qualifies
:   [prov:wasStartedBy](#wasStartedBy "http://www.w3.org/ns/prov#wasStartedBy") op

PROV-DM term
:   [Start](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Start "prov-dm")

### (40) Class: [prov:End](#End) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#End

End is when an activity is deemed to have been ended by an entity, known as trigger. The activity no longer exists after its end. Any usage, generation, or invalidation involving an activity precedes the activity's end. An end may refer to a trigger entity that terminated the activity, or to an activity, known as ender that generated the trigger.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

### End can be used to qualify wasEndedBy with time and location information.
### In this example, an experiment is stopped because an intermediate inconsitent resul.

:experiment 
   a prov:Activity;
   prov:wasEndedBy :inconsistentResult;
   prov:qualifiedEnd [
      a prov:End;
      prov:entity       :inconsistentResult;
      prov:atTime       "2011-07-16T01:52:02Z"^^xsd:dateTime;
      prov:atLocation   :scienceLab_003;
      prov:hadActivity  :analyse_intermediate_result ; 
   ];
.
   
:inconsistentResult a prov:Entity .

### An implicit process analyzes the intermediate result to confirm its expected consistency

analyse_intermediate_result
   a   prov:Activity ;
   prov:startedAtTime   "2011-07-15T12:52:02Z"^^xsd:dateTime;
   prov:endedAtTime     "2011-07-16T01:52:02Z"^^xsd:dateTime;
.
```

:   An instance of prov:End provides additional descriptions about the binary prov:wasEndedBy relation from some ended prov:Activity to an prov:Entity that ended it. For example, :ball\_game prov:wasEndedBy :buzzer; prov:qualifiedEnd [ a prov:End; prov:entity :buzzer; :foo :bar; prov:atTime '2012-03-09T08:05:08-05:00'^^xsd:dateTime ].

is subclass of
:   [prov:InstantaneousEvent](#InstantaneousEvent "http://www.w3.org/ns/prov#InstantaneousEvent")
    , [prov:EntityInfluence](#EntityInfluence "http://www.w3.org/ns/prov#EntityInfluence")

described with properties:

:   [prov:hadActivity](#hadActivity "http://www.w3.org/ns/prov#hadActivity") op
:   [prov:atTime](#atTime "http://www.w3.org/ns/prov#atTime") dp , [prov:entity](#p_entity "http://www.w3.org/ns/prov#entity") op

in range of
:   [prov:qualifiedEnd](#qualifiedEnd "http://www.w3.org/ns/prov#qualifiedEnd") op

qualifies
:   [prov:wasEndedBy](#wasEndedBy "http://www.w3.org/ns/prov#wasEndedBy") op

PROV-DM term
:   [End](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-End "prov-dm")

### (41) Class: [prov:Derivation](#Derivation) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Derivation

A derivation is a transformation of an entity into another, an update of an entity resulting in a new one, or the construction of a new entity based on a pre-existing entity.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

# The simplest (and least detailed) form of derivation.
:bar_chart 
   a prov:Entity;
   prov:wasDerivedFrom :aggregatedByRegions;  
.

# The simple form can be accompanied by a qualified form:
# which provides more details about how :bar_chart was 
# derived from :aggregatedRegions.

:bar_chart
   a prov:Entity;
   
   prov:wasDerivedFrom :aggregatedByRegions;  
   prov:qualifiedDerivation [                  
      a prov:Derivation;                      
      prov:entity      :aggregatedByRegions;  
                          
      # Derivations can cite the influencing Activity in doing the derivation.
      prov:hadActivity   :create_the_chart;

      # They can also cite the Usage and Generation that the Activity 
      # performed to generate :bar_chart.
      prov:hadUsage      :data_loading;
      prov:hadGeneration :plot_the_chart;
   ];
.

### The process during which the chart was created, from loading the data to the software, to process the data and plot the chart.
### Additional metadata was recorded, like when it started (before the usage), ended (after the generation of the chart) and who was associated with it.

:create_the_chart 
   a prov:Activity;
   prov:wasAssociatedWith :derek;
   prov:startedAtTime "2012-04-03T00:00:00Z"^^xsd:dateTime;
   prov:endedAtTime "2012-04-03T00:00:10Z"^^xsd:dateTime;
.

### The final chart was plotted

:plot_the_chart
   a prov:Generation, prov:InstantaneousEvent;
   prov:atTime "2012-04-03T00:00:01Z"^^xsd:dateTime;
.

### The data was getting used to create the chart

:data_loading
   a prov:Usage;
   prov:atTime "2012-04-03T00:00:00Z"^^xsd:dateTime;
.
```

:   The more specific forms of prov:Derivation (i.e., prov:Revision, prov:Quotation, prov:PrimarySource) should be asserted if they apply.
:   An instance of prov:Derivation provides additional descriptions about the binary prov:wasDerivedFrom relation from some derived prov:Entity to another prov:Entity from which it was derived. For example, :chewed\_bubble\_gum prov:wasDerivedFrom :unwrapped\_bubble\_gum; prov:qualifiedDerivation [ a prov:Derivation; prov:entity :unwrapped\_bubble\_gum; :foo :bar ].

is subclass of
:   [prov:EntityInfluence](#EntityInfluence "http://www.w3.org/ns/prov#EntityInfluence")

described with properties:

:   [prov:hadUsage](#hadUsage "http://www.w3.org/ns/prov#hadUsage") op
    , [prov:hadGeneration](#hadGeneration "http://www.w3.org/ns/prov#hadGeneration") op
:   [prov:hadActivity](#hadActivity "http://www.w3.org/ns/prov#hadActivity") op
:   [prov:entity](#p_entity "http://www.w3.org/ns/prov#entity") op

in range of
:   [prov:qualifiedDerivation](#qualifiedDerivation "http://www.w3.org/ns/prov#qualifiedDerivation") op

has subclasses
:   [prov:Revision](#Revision "http://www.w3.org/ns/prov#Revision")
    , [prov:PrimarySource](#PrimarySource "http://www.w3.org/ns/prov#PrimarySource")
    , [prov:Quotation](#Quotation "http://www.w3.org/ns/prov#Quotation")

qualifies
:   [prov:wasDerivedFrom](#wasDerivedFrom "http://www.w3.org/ns/prov#wasDerivedFrom") op

PROV-DM term
:   [Derivation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Derivation "prov-dm")

### (42) Class: [prov:PrimarySource](#PrimarySource) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#PrimarySource

A primary source for a topic refers to something produced by some agent with direct experience and knowledge about the topic, at the time of the topic's study, without benefit from hindsight.
Because of the directness of primary sources, they 'speak for themselves' in ways that cannot be captured through the filter of secondary sources. As such, it is important for secondary sources to reference those primary sources from which they were derived, so that their reliability can be investigated.
A primary source relation is a particular case of derivation of secondary materials from their primary sources. It is recognized that the determination of primary sources can be up to interpretation, and should be done according to conventions accepted within the application's domain.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:myPost 
   a prov:Entity;
   prov:hadPrimarySource :donQuixote;
   prov:qualifiedPrimarySource [
      a prov:PrimarySource;
      prov:entity :donQuixote;
      :confidenceValue "6"^^xsd:integer;
      rdfs:comment """Not sure if Don Quixote was the original source, 
                      so asserting a confidence value of 6 out of 10.""";
   ];
.

:donQuixote a prov:Entity.
```

:   An instance of prov:PrimarySource provides additional descriptions about the binary prov:hadPrimarySource relation from some secondary prov:Entity to an earlier, primary prov:Entity. For example, :blog prov:hadPrimarySource :newsArticle; prov:qualifiedPrimarySource [ a prov:PrimarySource; prov:entity :newsArticle; :foo :bar ] .

is subclass of
:   [prov:Derivation](#Derivation "http://www.w3.org/ns/prov#Derivation")

described with properties:

:   [prov:hadGeneration](#hadGeneration "http://www.w3.org/ns/prov#hadGeneration") op , [prov:hadUsage](#hadUsage "http://www.w3.org/ns/prov#hadUsage") op

in range of
:   [prov:qualifiedPrimarySource](#qualifiedPrimarySource "http://www.w3.org/ns/prov#qualifiedPrimarySource") op

qualifies
:   [prov:hadPrimarySource](#hadPrimarySource "http://www.w3.org/ns/prov#hadPrimarySource") op

PROV-DM term
:   [primary-source](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-primary-source "prov-dm")

### (43) Class: [prov:Quotation](#Quotation) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Quotation

A quotation is the repeat of (some or all of) an entity, such as text or image, by someone who may or may not be its original author. Quotation is a particular case of derivation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix ex:   <http://example.com/vocab#> .
@prefix :     <http://example.com/> .

:dagstuhl-quote
   a prov:Entity;
   prov:value   "why would people record and share provenance in the first place?";
   prov:wasQuotedFrom <http://purl.org/twc/page/thoughts-from-the-dagstuhl-workshop>;
   prov:qualifiedQuotation [
      a prov:Quotation;
      prov:entity     <http://purl.org/twc/page/thoughts-from-the-dagstuhl-workshop>;
      ex:fromSection 2;
   ];
   prov:wasAttributedTo <http://data.semanticweb.org/person/luc-moreau>;
.

<http://purl.org/twc/page/thoughts-from-the-dagstuhl-workshop> 
   a prov:Entity;
   prov:wasAttributedTo <http://data.semanticweb.org/person/paul-groth>;
.

<http://data.semanticweb.org/person/luc-moreau> a prov:Person, prov:Agent .
<http://data.semanticweb.org/person/paul-groth> a prov:Person, prov:Agent .
```

:   An instance of prov:Quotation provides additional descriptions about the binary prov:wasQuotedFrom relation from some taken prov:Entity from an earlier, larger prov:Entity. For example, :here\_is\_looking\_at\_you\_kid prov:wasQuotedFrom :casablanca\_script; prov:qualifiedQuotation [ a prov:Quotation; prov:entity :casablanca\_script; :foo :bar ].

is subclass of
:   [prov:Derivation](#Derivation "http://www.w3.org/ns/prov#Derivation")

described with properties:

:   [prov:hadGeneration](#hadGeneration "http://www.w3.org/ns/prov#hadGeneration") op , [prov:hadUsage](#hadUsage "http://www.w3.org/ns/prov#hadUsage") op

in range of
:   [prov:qualifiedQuotation](#qualifiedQuotation "http://www.w3.org/ns/prov#qualifiedQuotation") op

qualifies
:   [prov:wasQuotedFrom](#wasQuotedFrom "http://www.w3.org/ns/prov#wasQuotedFrom") op

PROV-DM term
:   [quotation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-quotation "prov-dm")

### (44) Class: [prov:Revision](#Revision) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Revision

A revision is a derivation for which the resulting entity is a revised version of some original. The implication here is that the resulting entity contains substantial content from the original. Revision is a particular case of derivation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix ex:   <http://example.com/vocab#> .
@prefix :     <http://example.com/> .

:draft2 
   a prov:Entity;
   prov:wasRevisionOf    :draft1;
   prov:qualifiedRevision [
      a prov:Revision;
      prov:entity        :draft1;
      ex:peerReviewed     false;
   ];

   prov:wasAssociatedWith :edward;
   prov:qualifiedAssociation [
      a prov:Association;
      prov:agent          :edward;
      prov:hadRole        :editor;
   ];
.

:draft1 a prov:Entity .

:edward 
   a prov:Person, prov:Agent;
.
```

:   An instance of prov:Revision provides additional descriptions about the binary prov:wasRevisionOf relation from some newer prov:Entity to an earlier prov:Entity. For example, :draft\_2 prov:wasRevisionOf :draft\_1; prov:qualifiedRevision [ a prov:Revision; prov:entity :draft\_1; :foo :bar ].

is subclass of
:   [prov:Derivation](#Derivation "http://www.w3.org/ns/prov#Derivation")

described with properties:

:   [prov:hadGeneration](#hadGeneration "http://www.w3.org/ns/prov#hadGeneration") op , [prov:hadUsage](#hadUsage "http://www.w3.org/ns/prov#hadUsage") op

in range of
:   [prov:qualifiedRevision](#qualifiedRevision "http://www.w3.org/ns/prov#qualifiedRevision") op

qualifies
:   [prov:wasRevisionOf](#wasRevisionOf "http://www.w3.org/ns/prov#wasRevisionOf") op

PROV-DM term
:   [revision](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-revision "prov-dm")

### (45) Class: [prov:ActivityInfluence](#ActivityInfluence) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#ActivityInfluence

ActivitiyInfluence is the capacity of an activity to have an effect on the character, development, or behavior of another by means of generation, invalidation, communication, or other.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:bar_chart 
   a prov:Entity;
   prov:wasGeneratedBy      :illustrating;
   prov:qualifiedGeneration :making-bar-chart;
. 

:making-bar-chart
   a prov:Generation, 
     prov:ActivityInfluence;  ## Instances of Generation, Invalidation and Communication qualify
   prov:activity :illustrating; ## the influence of an Activity (cited by prov:activity)
   rdfs:comment "Ended up with bar chart as line chart looked ugly."@en;
.

:illustrating a prov:Activity .
```

:   It is not recommended that the type ActivityInfluence be asserted without also asserting one of its more specific subclasses.
:   ActivityInfluence provides additional descriptions of an Activity's binary influence upon any other kind of resource. Instances of ActivityInfluence use the prov:activity property to cite the influencing Activity.

is subclass of
:   [prov:Influence](#Influence "http://www.w3.org/ns/prov#Influence")

described with properties:

:   [prov:activity](#p_activity "http://www.w3.org/ns/prov#activity") op
:   [prov:hadRole](#hadRole "http://www.w3.org/ns/prov#hadRole") op , [prov:influencer](#influencer "http://www.w3.org/ns/prov#influencer") op , [prov:hadActivity](#hadActivity "http://www.w3.org/ns/prov#hadActivity") op

has subclasses
:   [prov:Generation](#Generation "http://www.w3.org/ns/prov#Generation")
    , [prov:Invalidation](#Invalidation "http://www.w3.org/ns/prov#Invalidation")
    , [prov:Communication](#Communication "http://www.w3.org/ns/prov#Communication")

### (46) Class: [prov:Generation](#Generation) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Generation

Generation is the completion of production of a new entity by an activity. This entity did not exist before generation and becomes available for usage after this generation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix bbc:  <http://www.bbc.co.uk/> .
@prefix eg:   <http://example.com/vocab#> .
@prefix :     <http://example.com/> .

:bbcNews2012-04-03 
   a prov:Entity, eg:DailyNews;
   rdfs:comment """The BBC news home page on 2012-04-03 contained a reference 
                   to a given news item, but the BBC news home page on 
                   the next day did not.""";

   prov:wasGeneratedBy :publishingActivity;
   prov:qualifiedGeneration [
      a prov:Generation, prov:InstantaneousEvent;
      prov:atTime "2012-04-03T00:00:01Z"^^xsd:dateTime;
      prov:activity :publishingActivity;
   ];
   prov:qualifiedInvalidation [
      a prov:Invalidation, prov:InstantaneousEvent;
      prov:atTime "2012-04-03T23:59:59Z"^^xsd:dateTime;
   ];
.

:publishingActivity 
   a prov:Activity;
.
```

:   An instance of prov:Generation provides additional descriptions about the binary prov:wasGeneratedBy relation from a generated prov:Entity to the prov:Activity that generated it. For example, :cake prov:wasGeneratedBy :baking; prov:qualifiedGeneration [ a prov:Generation; prov:activity :baking; :foo :bar ].

is subclass of
:   [prov:InstantaneousEvent](#InstantaneousEvent "http://www.w3.org/ns/prov#InstantaneousEvent")
    , [prov:ActivityInfluence](#ActivityInfluence "http://www.w3.org/ns/prov#ActivityInfluence")

described with properties:

:   [prov:activity](#p_activity "http://www.w3.org/ns/prov#activity") op , [prov:atTime](#atTime "http://www.w3.org/ns/prov#atTime") dp

in range of
:   [prov:hadGeneration](#hadGeneration "http://www.w3.org/ns/prov#hadGeneration") op[prov:qualifiedGeneration](#qualifiedGeneration "http://www.w3.org/ns/prov#qualifiedGeneration") op

qualifies
:   [prov:wasGeneratedBy](#wasGeneratedBy "http://www.w3.org/ns/prov#wasGeneratedBy") op

PROV-DM term
:   [Generation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Generation "prov-dm")

### (47) Class: [prov:Communication](#Communication) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Communication

Communication is the exchange of an entity by two activities, one activity using the entity generated by the other.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix ex:   <http://example.com/vocab#> .
@prefix :     <http://example.com/> .

:writing-celebrity-gossip 
   a prov:Activity;
   prov:wasInformedBy          :voicemail-interception;
   prov:qualifiedCommunication :informing-the-journalist;
.

:informing-the-journalist 
   a prov:Communication;
   prov:activity   :voicemail-interception;
   ex:mediaType "email";
.

:voicemail-interception a prov:Activity .
```

:   An instance of prov:Communication provides additional descriptions about the binary prov:wasInformedBy relation from an informed prov:Activity to the prov:Activity that informed it. For example, :you\_jumping\_off\_bridge prov:wasInformedBy :everyone\_else\_jumping\_off\_bridge; prov:qualifiedCommunication [ a prov:Communication; prov:activity :everyone\_else\_jumping\_off\_bridge; :foo :bar ].

is subclass of
:   [prov:ActivityInfluence](#ActivityInfluence "http://www.w3.org/ns/prov#ActivityInfluence")

described with properties:

:   [prov:activity](#p_activity "http://www.w3.org/ns/prov#activity") op

in range of
:   [prov:qualifiedCommunication](#qualifiedCommunication "http://www.w3.org/ns/prov#qualifiedCommunication") op

qualifies
:   [prov:wasInformedBy](#wasInformedBy "http://www.w3.org/ns/prov#wasInformedBy") op

PROV-DM term
:   [Communication](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Communication "prov-dm")

### (48) Class: [prov:Invalidation](#Invalidation) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Invalidation

Invalidation is the start of the destruction, cessation, or expiry of an existing entity by an activity. The entity is no longer available for use (or further invalidation) after invalidation. Any generation or usage of an entity precedes its invalidation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix wgs:  <http://www.w3.org/2003/01/geo/wgs84_pos#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix :     <http://example.com/> .

:the-Painter 
   a prov:Entity, :Painting;
   rdfs:label "Le Peintre"@fr, "The Painter"@en;
   prov:wasAttributedTo <http://dbpedia.org/resource/Pablo_Picasso>;

   prov:wasInvalidatedBy :swissair_Flight_111_crash;
   prov:qualifiedInvalidation [
      a prov:Invalidation;
      prov:activity    :swissair_Flight_111_crash;
      prov:atTime      "1998-09-02T01:31:00Z"^^xsd:dateTime;
      prov:atLocation  <http://purl.org/twc/location/Swissair-Flight-111-crash>;
   ];
. 

<http://purl.org/twc/location/Swissair-Flight-111-crash>
   a prov:Location;
   wgs:lat   44.409167;
   wgs:long -63.973611;
.

<http://dbpedia.org/resource/Pablo_Picasso> 
   a prov:Agent;
   foaf:depiction <http://upload.wikimedia.org/wikipedia/commons/9/98/Pablo_picasso_1.jpg>;
.

:swissair_Flight_111_crash 
   a prov:Activity;
   prov:used          <http://dbpedia.org/resource/Swissair_Flight_111>;
   prov:startedAtTime "1998-09-02T01:31:00Z"^^xsd:dateTime;
   prov:atLocation    <http://dbpedia.org/resource/Atlantic_ocean>;
.
```

:   An instance of prov:Invalidation provides additional descriptions about the binary prov:wasInvalidatedBy relation from an invalidated prov:Entity to the prov:Activity that invalidated it. For example, :uncracked\_egg prov:wasInvalidatedBy :baking; prov:qualifiedInvalidation [ a prov:Invalidation; prov:activity :baking; :foo :bar ].

is subclass of
:   [prov:InstantaneousEvent](#InstantaneousEvent "http://www.w3.org/ns/prov#InstantaneousEvent")
    , [prov:ActivityInfluence](#ActivityInfluence "http://www.w3.org/ns/prov#ActivityInfluence")

described with properties:

:   [prov:activity](#p_activity "http://www.w3.org/ns/prov#activity") op , [prov:atTime](#atTime "http://www.w3.org/ns/prov#atTime") dp

in range of
:   [prov:qualifiedInvalidation](#qualifiedInvalidation "http://www.w3.org/ns/prov#qualifiedInvalidation") op

qualifies
:   [prov:wasInvalidatedBy](#wasInvalidatedBy "http://www.w3.org/ns/prov#wasInvalidatedBy") op

PROV-DM term
:   [Invalidation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Invalidation "prov-dm")

### (49) Class: [prov:AgentInfluence](#AgentInfluence) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#AgentInfluence

AgentInfluence is the capacity of an agent to have an effect on the character, development, or behavior of another by means of attribution, association, delegation, or other.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:illustrating
    a prov:Activity; 
    prov:wasAssociatedWith :derek;
    prov:qualifiedAssociation [
       a prov:Association, 
         prov:AgentInfluence; ## Instances of Generation, Invalidation and Communication qualify
       prov:agent   :derek;   ## the influence of an Agent (cited by prov:agent)
       prov:hadRole :illustrationist
    ];
.

:derek a prov:Person, prov:Agent, prov:Entity .

:illustratonist a prov:Role .
```

:   AgentInfluence provides additional descriptions of an Agent's binary influence upon any other kind of resource. Instances of AgentInfluence use the prov:agent property to cite the influencing Agent.
:   It is not recommended that the type AgentInfluence be asserted without also asserting one of its more specific subclasses.

is subclass of
:   [prov:Influence](#Influence "http://www.w3.org/ns/prov#Influence")

described with properties:

:   [prov:agent](#p_agent "http://www.w3.org/ns/prov#agent") op
:   [prov:hadRole](#hadRole "http://www.w3.org/ns/prov#hadRole") op , [prov:influencer](#influencer "http://www.w3.org/ns/prov#influencer") op , [prov:hadActivity](#hadActivity "http://www.w3.org/ns/prov#hadActivity") op

has subclasses
:   [prov:Delegation](#Delegation "http://www.w3.org/ns/prov#Delegation")
    , [prov:Association](#Association "http://www.w3.org/ns/prov#Association")
    , [prov:Attribution](#Attribution "http://www.w3.org/ns/prov#Attribution")

### (50) Class: [prov:Attribution](#Attribution) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Attribution

Attribution is the ascribing of an entity to an agent.
When an entity e is attributed to agent ag, entity e was generated by some unspecified activity that in turn was associated to agent ag. Thus, this relation is useful when the activity is not known, or irrelevant.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix ex:   <http://example.com/vocab#> .
@prefix :     <http://example.com/> .

<http://dbpedia.org/resource/Fallingwater>
   a prov:Entity;
   prov:wasAttributedTo <http://dbpedia.org/resource/Edgar_J._Kaufmann>,
                        <http://dbpedia.org/resource/Frank_Lloyd_Wright>,
                        :western-Pennsylvania-Conservancy;
   prov:qualifiedAttribution [
      a prov:Attribution;
      prov:agent <http://dbpedia.org/resource/Edgar_J._Kaufmann>;
      ex:hadRole :owner;
   ];
   prov:qualifiedAttribution [
      a prov:Attribution;
      prov:agent <http://dbpedia.org/resource/Frank_Lloyd_Wright>;
      ex:hadRole :architect;
   ];
   prov:qualifiedAttribution [
      a prov:Attribution;
      prov:agent :western-Pennsylvania-Conservancy;
      ex:hadRole :conserver;
   ];
.

<http://dbpedia.org/resource/Edgar_J._Kaufmann>  a prov:Person, prov:Agent .
<http://dbpedia.org/resource/Frank_Lloyd_Wright> a prov:Person, prov:Agent .
:western-Pennsylvania-Conservancy a prov:Organization, prov:Agent .
```

:   An instance of prov:Attribution provides additional descriptions about the binary prov:wasAttributedTo relation from an prov:Entity to some prov:Agent that had some responsible for it. For example, :cake prov:wasAttributedTo :baker; prov:qualifiedAttribution [ a prov:Attribution; prov:entity :baker; :foo :bar ].

is subclass of
:   [prov:AgentInfluence](#AgentInfluence "http://www.w3.org/ns/prov#AgentInfluence")

described with properties:

:   [prov:agent](#p_agent "http://www.w3.org/ns/prov#agent") op

in range of
:   [prov:qualifiedAttribution](#qualifiedAttribution "http://www.w3.org/ns/prov#qualifiedAttribution") op

qualifies
:   [prov:wasAttributedTo](#wasAttributedTo "http://www.w3.org/ns/prov#wasAttributedTo") op

PROV-DM term
:   [attribution](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-attribution "prov-dm")

### (51) Class: [prov:Association](#Association) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Association

An activity association is an assignment of responsibility to an agent for an activity, indicating that the agent had a role in the activity. It further allows for a plan to be specified, which is the plan intended by the agent to achieve some goals in the context of this activity.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:illustrating
    a prov:Activity; 
    prov:wasAssociatedWith :derek, 
                           :steve;
    prov:qualifiedAssociation [
        a prov:Association;
        prov:agent   :derek;
        prov:hadRole :illustrationist;
    ];
    prov:qualifiedAssociation [
        a prov:Association;
        prov:agent   :steve;
        prov:hadRole :stylist;
        prov:hadPlan :style-guide;
        rdfs:comment "Steve helped Derek conform with the publisher's style guide."@en;
    ];
.

:derek a prov:Person, prov:Agent, prov:Entity .
:steve a prov:Person, prov:Agent, prov:Entity .

:illustratonist a prov:Role .
:stylist        a prov:Role .

:style-guide a prov:Plan, prov:Entity .
```

:   An instance of prov:Association provides additional descriptions about the binary prov:wasAssociatedWith relation from an prov:Activity to some prov:Agent that had some responsiblity for it. For example, :baking prov:wasAssociatedWith :baker; prov:qualifiedAssociation [ a prov:Association; prov:agent :baker; :foo :bar ].

is subclass of
:   [prov:AgentInfluence](#AgentInfluence "http://www.w3.org/ns/prov#AgentInfluence")

described with properties:

:   [prov:hadPlan](#hadPlan "http://www.w3.org/ns/prov#hadPlan") op
:   [prov:hadRole](#hadRole "http://www.w3.org/ns/prov#hadRole") op
:   [prov:agent](#p_agent "http://www.w3.org/ns/prov#agent") op

in range of
:   [prov:qualifiedAssociation](#qualifiedAssociation "http://www.w3.org/ns/prov#qualifiedAssociation") op

qualifies
:   [prov:wasAssociatedWith](#wasAssociatedWith "http://www.w3.org/ns/prov#wasAssociatedWith") op

PROV-DM term
:   [Association](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Association "prov-dm")

### (52) Class: [prov:Plan](#Plan) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Plan

A plan is an entity that represents a set of actions or steps intended by one or more agents to achieve some goals.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:illustrating 
   a prov:Activity;
   prov:qualifiedAssociation :steve-checking-style-guide;
.

:steve-checking-style-guide
   a prov:Association;
   prov:agent   :steve;
   prov:hadPlan :style-guide;
   rdfs:comment "Steve followed the publisher's style guide"@en;
. 

:style-guide
   a prov:Plan, prov:Entity;
   rdfs:comment "Use blue graphs for positive spin, red for negative"@en;
.
```

:   There exist no prescriptive requirement on the nature of plans, their representation, the actions or steps they consist of, or their intended goals. Since plans may evolve over time, it may become necessary to track their provenance, so plans themselves are entities. Representing the plan explicitly in the provenance can be useful for various tasks: for example, to validate the execution as represented in the provenance record, to manage expectation failures, or to provide explanations.

is subclass of
:   [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

in range of
:   [prov:hadPlan](#hadPlan "http://www.w3.org/ns/prov#hadPlan") op

PROV-DM term
:   [Association](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Association "prov-dm")

### (53) Class: [prov:Delegation](#Delegation) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Delegation

Delegation is the assignment of authority and responsibility to an agent (by itself or by another agent) to carry out a specific activity as a delegate or representative, while the agent it acts on behalf of retains some responsibility for the outcome of the delegated work.
For example, a student acted on behalf of his supervisor, who acted on behalf of the department chair, who acted on behalf of the university; all those agents are responsible in some way for the activity that took place but we do not say explicitly who bears responsibility and to what degree.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix ex:   <http://example.com/vocab#> .
@prefix :     <http://example.com/> .

### In this example, Frank (an insurance agent) acts on behalf of his company for performing
### a policy sale
 
:policySale
   a prov:Activity;
   prov:wasAssociatedWith :insuranceAgent_Frank;
.

:insuranceAgent_Frank
   a prov:Person;
   prov:actedOnBehalfOf :insuranceCompany_A;
   prov:qualifiedDelegation [
      a prov:Delegation;
      prov:agent        :insuranceCompany_A;
      ex:rewardScheme   "commission";
      prov:hadActivity  :policySale ;
   ];
.
```

:   An instance of prov:Delegation provides additional descriptions about the binary prov:actedOnBehalfOf relation from a performing prov:Agent to some prov:Agent for whom it was performed. For example, :mixing prov:wasAssociatedWith :toddler . :toddler prov:actedOnBehalfOf :mother; prov:qualifiedDelegation [ a prov:Delegation; prov:entity :mother; :foo :bar ].

is subclass of
:   [prov:AgentInfluence](#AgentInfluence "http://www.w3.org/ns/prov#AgentInfluence")

described with properties:

:   [prov:hadActivity](#hadActivity "http://www.w3.org/ns/prov#hadActivity") op
:   [prov:agent](#p_agent "http://www.w3.org/ns/prov#agent") op

in range of
:   [prov:qualifiedDelegation](#qualifiedDelegation "http://www.w3.org/ns/prov#qualifiedDelegation") op

qualifies
:   [prov:actedOnBehalfOf](#actedOnBehalfOf "http://www.w3.org/ns/prov#actedOnBehalfOf") op

PROV-DM term
:   [delegation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-delegation "prov-dm")

### (54) Class: [prov:InstantaneousEvent](#InstantaneousEvent) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#InstantaneousEvent

The PROV data model is implicitly based on a notion of instantaneous events (or just events), that mark transitions in the world. Events include generation, usage, or invalidation of entities, as well as starting or ending of activities. This notion of event is not first-class in the data model, but it is useful for explaining its other concepts and its semantics.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix bbc:  <http://www.bbc.co.uk/> .
@prefix :     <http://example.com/> .

:bbcNews2012-04-03 
   a prov:Entity, :DailyNews;
   rdfs:comment """The BBC news home page on 2012-04-03 contained 
                   a reference to a given news item, but the BBC news 
                   home page on the next day did not.""";

   prov:qualifiedGeneration [
      a prov:Generation, prov:InstantaneousEvent;
      prov:atTime "2012-04-03T00:00:01Z"^^xsd:dateTime;
   ];
   prov:qualifiedInvalidation [
      a prov:Invalidation, prov:InstantaneousEvent;
      prov:atTime "2012-04-03T23:59:59Z"^^xsd:dateTime;
   ];
.
```

:   An instantaneous event, or event for short, happens in the world and marks a change in the world, in its activities and in its entities. The term 'event' is commonly used in process algebra with a similar meaning. Events represent communications or interactions; they are assumed to be atomic and instantaneous.

described with properties:

:   [prov:atTime](#atTime "http://www.w3.org/ns/prov#atTime") dp
:   [prov:hadRole](#hadRole "http://www.w3.org/ns/prov#hadRole") op , [prov:atLocation](#atLocation "http://www.w3.org/ns/prov#atLocation") op

has subclasses
:   [prov:Generation](#Generation "http://www.w3.org/ns/prov#Generation")
    , [prov:Start](#Start "http://www.w3.org/ns/prov#Start")
    , [prov:Invalidation](#Invalidation "http://www.w3.org/ns/prov#Invalidation")
    , [prov:End](#End "http://www.w3.org/ns/prov#End")
    , [prov:Usage](#Usage "http://www.w3.org/ns/prov#Usage")

### (55) Class: [prov:Role](#Role) back to [qualified classes](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#Role

A role is the function of an entity or agent with respect to an activity, in the context of a usage, generation, invalidation, association, start, and end.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:divideActivity 
   a prov:Activity;   
   prov:used :variableA, :variableB;

   prov:qualifiedUsage [
      a prov:Usage;
      prov:entity  :variableA;
      prov:hadRole :dividend;          
   ];
   prov:qualifiedUsage [
      a prov:Usage;
      prov:entity  :variableB;
      prov:hadRole :divisor;          
   ];

   prov:generated :result_112234;
.

:variableA 
   a prov:Entity;
   prov:value 10;
.
:variableB 
   a prov:Entity;
   prov:value 2;
.

:dividend a prov:Role.
:divisor  a prov:Role.

:result_112234 
   a prov:Entity;
   prov:value 5;
   prov:wasGeneratedBy :divideActivity;
.
```

in range of
:   [prov:hadRole](#hadRole "http://www.w3.org/ns/prov#hadRole") op

PROV-DM term
:   [attribute-role](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-attribute-role "prov-dm")

### (56) Property: [prov:wasInfluencedBy](#wasInfluencedBy) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#wasInfluencedBy

Influence is the capacity of an entity, activity, or agent to have an effect on the character, development, or behavior of another by means of usage, start, end, generation, invalidation, communication, derivation, attribution, association, or delegation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:illustrationActivity 
   a prov:Activity;
   prov:used              :aggregatedByRegions;
   prov:wasAssociatedWith :derek;
   prov:wasInformedBy     :aggregationActivity;
.

:illustrationActivity 
   a prov:Activity;
   prov:wasInfluencedBy :aggregatedByRegions, # prov:wasInfluencedBy is a superproperty of
                        :derek,               # many of the direct binary
                        :aggregationActivity; # PROV-O properties.
.

:aggregationActivity a prov:Activity .  
:derek               a prov:Agent .
:aggregatedByRegions a prov:Entity .
```

Because prov:wasInfluencedBy is a broad relation, its more specific subproperties (e.g. prov:wasInformedBy, prov:actedOnBehalfOf, prov:wasEndedBy, etc.) should be used when applicable.

This property has multiple RDFS domains to suit multiple OWL Profiles. See [PROV-O OWL Profile](#owl-profile).

has domain
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")
      or [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent")
      or [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")
      or [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent")
      or [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has sub-properties
:   * [prov:hadMember](#hadMember "http://www.w3.org/ns/prov#hadMember")
    * [prov:wasAttributedTo](#wasAttributedTo "http://www.w3.org/ns/prov#wasAttributedTo")
    * [prov:wasAssociatedWith](#wasAssociatedWith "http://www.w3.org/ns/prov#wasAssociatedWith")
    * [prov:wasGeneratedBy](#wasGeneratedBy "http://www.w3.org/ns/prov#wasGeneratedBy")
    * [prov:wasDerivedFrom](#wasDerivedFrom "http://www.w3.org/ns/prov#wasDerivedFrom")
    * [prov:wasInvalidatedBy](#wasInvalidatedBy "http://www.w3.org/ns/prov#wasInvalidatedBy")
    * [prov:used](#used "http://www.w3.org/ns/prov#used")
    * [prov:actedOnBehalfOf](#actedOnBehalfOf "http://www.w3.org/ns/prov#actedOnBehalfOf")
    * [prov:wasInformedBy](#wasInformedBy "http://www.w3.org/ns/prov#wasInformedBy")
    * [prov:wasStartedBy](#wasStartedBy "http://www.w3.org/ns/prov#wasStartedBy")
    * [prov:wasEndedBy](#wasEndedBy "http://www.w3.org/ns/prov#wasEndedBy")

can be qualified with
:   * [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op
    * [prov:Influence](#Influence "http://www.w3.org/ns/prov#Influence")

PROV-DM term
:   [influence](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-influence "prov-dm")

### (57) Property: [prov:qualifiedInfluence](#qualifiedInfluence) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#qualifiedInfluence

Influence is the capacity of an entity, activity, or agent to have an effect on the character, development, or behavior of another by means of usage, start, end, generation, invalidation, communication, derivation, attribution, association, or delegation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix my:   <http://example.com/ontology#> .
@prefix :     <http://example.com/> .

# Although domain extension 'my:wasConductedBy' is not defined by PROV-O,
# the relation between a surgery and an agent can still be qualified
# by reusing prov:Influence and one of its three subclasses:
# AgentInfluence, EntityInfluence, and ActivityInfluence
# (depending on the type of the influencing object).

:conductingSurgery_1
   a prov:Activity;

   # This unqualified influence is unknown in PROV;
   # it would be a subproperty of prov:wasAssociatedWith.
   my:wasConductedBy    :bob;
   prov:wasInfluencedBy :bob;  
   prov:qualifiedInfluence [   
      # Even though PROV systems do not understand my:wasConductedBy, 
      # they will at least understand that :bob influenced the 
      # surgery in some way.
      a prov:Influence;      # Inferred
      prov:agent   :bob;     # The object of my:wasConductedBy

      # Domain extension properties may be used to describe the
      # influences that an Entity, Activity, or Agent
      # have upon another Entity, Activity, or Agent.
      my:degree .72;
   ];
.

:bob a prov:Agent .
```

Because prov:qualifiedInfluence is a broad relation, the more specific relations (qualifiedCommunication, qualifiedDelegation, qualifiedEnd, etc.) should be used when applicable.

has domain
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")
      or [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent")
      or [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:Influence](#Influence "http://www.w3.org/ns/prov#Influence")

has sub-properties
:   * [prov:qualifiedAssociation](#qualifiedAssociation "http://www.w3.org/ns/prov#qualifiedAssociation")
    * [prov:qualifiedRevision](#qualifiedRevision "http://www.w3.org/ns/prov#qualifiedRevision")
    * [prov:qualifiedInvalidation](#qualifiedInvalidation "http://www.w3.org/ns/prov#qualifiedInvalidation")
    * [prov:qualifiedPrimarySource](#qualifiedPrimarySource "http://www.w3.org/ns/prov#qualifiedPrimarySource")
    * [prov:qualifiedDerivation](#qualifiedDerivation "http://www.w3.org/ns/prov#qualifiedDerivation")
    * [prov:qualifiedGeneration](#qualifiedGeneration "http://www.w3.org/ns/prov#qualifiedGeneration")
    * [prov:qualifiedUsage](#qualifiedUsage "http://www.w3.org/ns/prov#qualifiedUsage")
    * [prov:qualifiedQuotation](#qualifiedQuotation "http://www.w3.org/ns/prov#qualifiedQuotation")
    * [prov:qualifiedStart](#qualifiedStart "http://www.w3.org/ns/prov#qualifiedStart")
    * [prov:qualifiedAttribution](#qualifiedAttribution "http://www.w3.org/ns/prov#qualifiedAttribution")
    * [prov:qualifiedEnd](#qualifiedEnd "http://www.w3.org/ns/prov#qualifiedEnd")
    * [prov:qualifiedCommunication](#qualifiedCommunication "http://www.w3.org/ns/prov#qualifiedCommunication")
    * [prov:qualifiedDelegation](#qualifiedDelegation "http://www.w3.org/ns/prov#qualifiedDelegation")

qualifies
:   [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy") op

PROV-DM term
:   [influence](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-influence "prov-dm")

### (58) Property: [prov:qualifiedGeneration](#qualifiedGeneration) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#qualifiedGeneration

Generation is the completion of production of a new entity by an activity. This entity did not exist before generation and becomes available for usage after this generation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:bar_chart
   a prov:Entity;
   prov:wasGeneratedBy :illustrating;
   prov:qualifiedGeneration [
      a prov:Generation;
      prov:activity :illustrating;
      rdfs:comment "Ended up with bar chart as line chart looked ugly."@en;
   ];
.

:illustrating a prov:Activity .
```

If this Activity prov:generated Entity :e, then it can qualify how it performed the Generation using prov:qualifiedGeneration [ a prov:Generation; prov:entity :e; :foo :bar ].

has super-properties
:   * [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:Generation](#Generation "http://www.w3.org/ns/prov#Generation")

qualifies
:   [prov:wasGeneratedBy](#wasGeneratedBy "http://www.w3.org/ns/prov#wasGeneratedBy") op

PROV-DM term
:   [Generation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Generation "prov-dm")

### (59) Property: [prov:qualifiedDerivation](#qualifiedDerivation) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#qualifiedDerivation

A derivation is a transformation of an entity into another, an update of an entity resulting in a new one, or the construction of a new entity based on a pre-existing entity.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:bar_chart
   a prov:Entity;
   prov:wasDerivedFrom :aggregatedByRegions;
   prov:qualifiedDerivation [
      a prov:Derivation;
      prov:entity :aggregatedByRegions; 
      
      ## More details about the activity underpinning the derivation        
      prov:hadGeneration :chat_plotting; 
      prov:hadActivity   :chart_creation ;
   ];
.

### The process of creating the chart, from loading the data, to process it, and plot it to end users

:chart_creation
    a    prov:Activity ;
    prov:wasAssociatedWith :derek;
    prov:startedAtTime  "2011-07-16T01:52:02Z"^^xsd:dateTime;
    prov:endedAtTime "2011-07-16T03:00:02Z"^^xsd:dateTime;
.

#### Now the chart is plotted

:chat_plotting
    a    prov:Generation ;
    prov:atTime    "2011-07-16T03:00:02Z"^^xsd:dateTime;
.
```

If this Entity prov:wasDerivedFrom Entity :e, then it can qualify how it was derived using prov:qualifiedDerivation [ a prov:Derivation; prov:entity :e; :foo :bar ].

has super-properties
:   * [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:Derivation](#Derivation "http://www.w3.org/ns/prov#Derivation")

qualifies
:   [prov:wasDerivedFrom](#wasDerivedFrom "http://www.w3.org/ns/prov#wasDerivedFrom") op

PROV-DM term
:   [Derivation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Derivation "prov-dm")

### (60) Property: [prov:qualifiedPrimarySource](#qualifiedPrimarySource) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#qualifiedPrimarySource

A primary source for a topic refers to something produced by some agent with direct experience and knowledge about the topic, at the time of the topic's study, without benefit from hindsight.
Because of the directness of primary sources, they 'speak for themselves' in ways that cannot be captured through the filter of secondary sources. As such, it is important for secondary sources to reference those primary sources from which they were derived, so that their reliability can be investigated.
A primary source relation is a particular case of derivation of secondary materials from their primary sources. It is recognized that the determination of primary sources can be up to interpretation, and should be done according to conventions accepted within the application's domain.

**Example**

```
@prefix rdfs:    <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:     <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:     <http://www.w3.org/2002/07/owl#> .
@prefix dcterms: <http://purl.org/dc/terms/> .
@prefix prov:    <http://www.w3.org/ns/prov#> .
@prefix ex:      <http://example.com/vocab#> .
@prefix :        <http://example.com/> .

:temperatureDisplay
   a prov:Entity;
   prov:hadPrimarySource :sensorReading20120510;
   prov:qualifiedPrimarySource [
      a prov:PrimarySource;
      prov:entity        :sensorReading20120510;
      ex:precisionLoss true;
      rdfs:comment """The displayed temperature does not show the full precision 
                      available in the reading.""";
   ];
.

:sensorReading20120510 
   a prov:Entity;
   prov:wasGeneratedBy :temperatureSensor;
.
```

If this Entity prov:hadPrimarySource Entity :e, then it can qualify how using prov:qualifiedPrimarySource [ a prov:PrimarySource; prov:entity :e; :foo :bar ].

has super-properties
:   * [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:PrimarySource](#PrimarySource "http://www.w3.org/ns/prov#PrimarySource")

qualifies
:   [prov:hadPrimarySource](#hadPrimarySource "http://www.w3.org/ns/prov#hadPrimarySource") op

PROV-DM term
:   [primary-source](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-primary-source "prov-dm")

### (61) Property: [prov:qualifiedQuotation](#qualifiedQuotation) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#qualifiedQuotation

A quotation is the repeat of (some or all of) an entity, such as text or image, by someone who may or may not be its original author. Quotation is a particular case of derivation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix my:   <http://example.com/vocab/my#> .
@prefix :     <http://example.com/> .

:bl-dagstuhl
   a prov:Entity;
   prov:value """During the workshop, it became clear to me that the consensus
   based models (which are often graphical in nature) can not only be
   formalized but also be directly connected to these database focused
   formalizations. I just needed to get over the differences in syntax.
   This could imply that we could have nice way to trace provenance across
   systems and through databases and be able to understand the
   mathematical properties of this interconnection.""";
   prov:wasQuotedFrom <http://purl.org/twc/page/thoughts-from-the-dagstuhl-workshop>;
   prov:qualifiedQuotation [
      a prov:Quotation;
      prov:entity <http://purl.org/twc/page/thoughts-from-the-dagstuhl-workshop>;
      my:fromSection 1;
   ];
.

<http://purl.org/twc/page/thoughts-from-the-dagstuhl-workshop>
   a prov:Entity;
   prov:wasAttributedTo <http://data.semanticweb.org/person/paul-groth>;
.

<http://data.semanticweb.org/person/luc-moreau> a prov:Person, prov:Agent .
<http://data.semanticweb.org/person/paul-groth> a prov:Person, prov:Agent .
```

If this Entity prov:wasQuotedFrom Entity :e, then it can qualify how using prov:qualifiedQuotation [ a prov:Quotation; prov:entity :e; :foo :bar ].

has super-properties
:   * [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:Quotation](#Quotation "http://www.w3.org/ns/prov#Quotation")

qualifies
:   [prov:wasQuotedFrom](#wasQuotedFrom "http://www.w3.org/ns/prov#wasQuotedFrom") op

PROV-DM term
:   [quotation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-quotation "prov-dm")

### (62) Property: [prov:qualifiedRevision](#qualifiedRevision) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#qualifiedRevision

A revision is a derivation for which the resulting entity is a revised version of some original. The implication here is that the resulting entity contains substantial content from the original. Revision is a particular case of derivation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:draft2 
   a prov:Entity;
   prov:wasRevisionOf :draft1;
   prov:qualifiedRevision [
      a prov:Revision;
      prov:entity :draft1
   ];
   prov:wasAttributedTo :eddie;
.

:draft1 a prov:Entity .
:eddie  a prov:Person, prov:Agent, prov:Entity .
```

If this Entity prov:wasRevisionOf Entity :e, then it can qualify how it was revised using prov:qualifiedRevision [ a prov:Revision; prov:entity :e; :foo :bar ].

has super-properties
:   * [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:Revision](#Revision "http://www.w3.org/ns/prov#Revision")

qualifies
:   [prov:wasRevisionOf](#wasRevisionOf "http://www.w3.org/ns/prov#wasRevisionOf") op

PROV-DM term
:   [revision](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-revision "prov-dm")

### (63) Property: [prov:qualifiedAttribution](#qualifiedAttribution) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#qualifiedAttribution

Attribution is the ascribing of an entity to an agent.
When an entity e is attributed to agent ag, entity e was generated by some unspecified activity that in turn was associated to agent ag. Thus, this relation is useful when the activity is not known, or irrelevant.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix ex:   <http://example.com/vocab#> .
@prefix :     <http://example.com/> .

## When the role of the agent is not known or does not matter:

:nationalRegionsList 
   a prov:Entity;
   prov:wasAttributedTo :civil_action_group;
.

:civil_action_group a prov:Agent .

## If we want to express the role of the agent:

:nationalRegionsList 
   a prov:Entity;
   prov:qualifiedAttribution [
      a prov:Attribution;
      prov:agent :civil_action_group;
      ex:hadRole :owner;
   ]
.
```

If this Entity prov:wasAttributedTo Agent :ag, then it can qualify how it was influenced using prov:qualifiedAttribution [ a prov:Attribution; prov:agent :ag; :foo :bar ].

has super-properties
:   * [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:Attribution](#Attribution "http://www.w3.org/ns/prov#Attribution")

qualifies
:   [prov:wasAttributedTo](#wasAttributedTo "http://www.w3.org/ns/prov#wasAttributedTo") op

PROV-DM term
:   [attribution](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-attribution "prov-dm")

### (64) Property: [prov:qualifiedInvalidation](#qualifiedInvalidation) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#qualifiedInvalidation

Invalidation is the start of the destruction, cessation, or expiry of an existing entity by an activity. The entity is no longer available for use (or further invalidation) after invalidation. Any generation or usage of an entity precedes its invalidation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix ex:   <http://example.com/ontology#> .
@prefix :     <http://example.com/> .

:the-Painter 
   a prov:Entity, ex:Painting;
   rdfs:label "Le Peintre"@fr, "The Painter"@en;
   prov:wasAttributedTo <http://dbpedia.org/resource/Pablo_Picasso>;

   prov:wasInvalidatedBy :swissair_Flight_111_crash;
   prov:qualifiedInvalidation [
      a prov:Invalidation;
      prov:activity    :swissair_Flight_111_crash;
      prov:atTime      "1998-09-02T01:31:00Z"^^xsd:dateTime;
      prov:atLocation  <http://purl.org/twc/location/Swissair-Flight-111-crash>;
   ];
. 

:swissair_Flight_111_crash a prov:Activity .
<http://purl.org/twc/location/Swissair-Flight-111-crash> a prov:Location .
```

If this Entity prov:wasInvalidatedBy Activity :a, then it can qualify how it was invalidated using prov:qualifiedInvalidation [ a prov:Invalidation; prov:activity :a; :foo :bar ].

has super-properties
:   * [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op

has domain
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

has range
:   * [prov:Invalidation](#Invalidation "http://www.w3.org/ns/prov#Invalidation")

qualifies
:   [prov:wasInvalidatedBy](#wasInvalidatedBy "http://www.w3.org/ns/prov#wasInvalidatedBy") op

PROV-DM term
:   [Invalidation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Invalidation "prov-dm")

### (65) Property: [prov:qualifiedStart](#qualifiedStart) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#qualifiedStart

Start is when an activity is deemed to have been started by an entity, known as trigger. The activity did not exist before its start. Any usage, generation, or invalidation involving an activity follows the activity's start. A start may refer to a trigger entity that set off the activity, or to an activity, known as starter, that generated the trigger.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

# Start can be used to qualify wasStartedBy with time and location information.

:consistency_checking
   a prov:Activity;
   prov:wasStartedBy :updated_data_record;
   prov:qualifiedStart [
      a prov:Start;
      prov:entity       :updated_data_record;
      prov:atTime       "2011-07-06T01:48:36Z"^^xsd:dateTime;
      prov:atLocation   :scienceLab_003;
      prov:hadActivity  :syntax_checking;
   ];
.

:updated_data_record a prov:Entity .

### There is an explicit process of checking the syntax of the updated data record

:syntax_checking
   a   prov:Activity ;
   prov:startedAtTime      "2011-07-06T01:48:36Z"^^xsd:dateTime;
   prov:endedAtTime        "2011-07-06T02:12:36Z"^^xsd:dateTime;
   prov:wasAssociatedWith  :syntax_checker ;
.

:syntax_checker   a   prov:SoftwareAgent .
```

If this Activity prov:wasStartedBy Entity :e1, then it can qualify how it was started using prov:qualifiedStart [ a prov:Start; prov:entity :e1; :foo :bar ].

has super-properties
:   * [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op

has domain
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

has range
:   * [prov:Start](#Start "http://www.w3.org/ns/prov#Start")

qualifies
:   [prov:wasStartedBy](#wasStartedBy "http://www.w3.org/ns/prov#wasStartedBy") op

PROV-DM term
:   [Start](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Start "prov-dm")

### (66) Property: [prov:qualifiedUsage](#qualifiedUsage) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#qualifiedUsage

Usage is the beginning of utilizing an entity by an activity. Before usage, the activity had not begun to utilize this entity and could not have been affected by the entity.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix ex:   <http://example.com/vocab#> .
@prefix :     <http://example.com/> .

:newsPublication
   a prov:Activity;
   prov:used                    :tsunami_image;
   prov:qualifiedUsage [
      a prov:Usage;
      prov:entity               :tsunami_image;
      ex:hasCopyrightPermission :licensedUse;  
      ex:hasOwner               :reuters;
   ];
.

:tsunami_image a prov:Entity .
:reuters       a prov:Agent .
```

If this Activity prov:used Entity :e, then it can qualify how it used it using prov:qualifiedUsage [ a prov:Usage; prov:entity :e; :foo :bar ].

has super-properties
:   * [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op

has domain
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

has range
:   * [prov:Usage](#Usage "http://www.w3.org/ns/prov#Usage")

qualifies
:   [prov:used](#used "http://www.w3.org/ns/prov#used") op

PROV-DM term
:   [Usage](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Usage "prov-dm")

### (67) Property: [prov:qualifiedCommunication](#qualifiedCommunication) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#qualifiedCommunication

Communication is the exchange of an entity by two activities, one activity using the entity generated by the other.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:writing-celebrity-gossip 
   a prov:Activity;
   prov:wasAssociatedWith :journalist;
   prov:wasInformedBy     :voicemail-interception;
   prov:qualifiedCommunication [
      a prov:Communication;
      prov:activity :voicemail-interception;
      rdfs:comment """The journalist was informed by the private 
                      investigator, but we don't know how or what he was told."""@en;
   ];
.

:voicemail-interception 
   a prov:Activity;
   prov:wasAssociatedWith :private-investigator;
.

:private-investigator a prov:Agent .
:journalist           a prov:Agent .
```

If this Activity prov:wasInformedBy Activity :a, then it can qualify how it was influenced using prov:qualifiedCommunication [ a prov:Communication; prov:activity :a; :foo :bar ].

has super-properties
:   * [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op

has domain
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

has range
:   * [prov:Communication](#Communication "http://www.w3.org/ns/prov#Communication")

can be qualified with
:   * [prov:Communication](#Communication "http://www.w3.org/ns/prov#Communication")

PROV-DM term
:   [Communication](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Communication "prov-dm")

### (68) Property: [prov:qualifiedAssociation](#qualifiedAssociation) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#qualifiedAssociation

An activity association is an assignment of responsibility to an agent for an activity, indicating that the agent had a role in the activity. It further allows for a plan to be specified, which is the plan intended by the agent to achieve some goals in the context of this activity.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:illustrating
    a prov:Activity; 
    prov:wasAssociatedWith :derek, 
                           :steve;
    prov:qualifiedAssociation [
        a prov:Association;
        prov:agent   :derek;
        prov:hadRole :illustrationist;
        rdfs:comment "Derek made the illustration"@en
    ];
    prov:qualifiedAssociation [
        a prov:Association;
        prov:agent   :steve;
        prov:hadRole :stylist;
        prov:hadPlan :style-guide;
        rdfs:comment "Steve helped Derek conform with the publisher's style guide."@en 
    ];
    prov:qualifiedAssociation [
        a prov:Association;
        prov:agent   :derek;
        prov:hadRole :stylist;
        rdfs:comment "But Derek also did some styling of his own."@en 
    ] .

:derek a prov:Person, prov:Agent .
:steve a prov:Person, prov:Agent .

:illustratonist a prov:Role .
:stylist        a prov:Role .

:style-guide a prov:Plan, prov:Entity .
```

If this Activity prov:wasAssociatedWith Agent :ag, then it can qualify the Association using prov:qualifiedAssociation [ a prov:Association; prov:agent :ag; :foo :bar ].

has super-properties
:   * [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op

has domain
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

has range
:   * [prov:Association](#Association "http://www.w3.org/ns/prov#Association")

qualifies
:   [prov:wasAssociatedWith](#wasAssociatedWith "http://www.w3.org/ns/prov#wasAssociatedWith") op

PROV-DM term
:   [Association](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Association "prov-dm")

### (69) Property: [prov:qualifiedEnd](#qualifiedEnd) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#qualifiedEnd

End is when an activity is deemed to have been ended by an entity, known as trigger. The activity no longer exists after its end. Any usage, generation, or invalidation involving an activity precedes the activity's end. An end may refer to a trigger entity that terminated the activity, or to an activity, known as ender that generated the trigger.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

### End can be used to qualify wasEndedBy with time and location information

:experiment 
   a prov:Activity;
   prov:wasEndedBy :inconsistentResult;
   prov:qualifiedEnd [
      a prov:End;
      prov:entity       :inconsistentResult;
      prov:atTime       "2011-07-16T01:52:02Z"^^xsd:dateTime;
      prov:atLocation   :scienceLab_003;
      prov:hadActivity  :analyse_intermediate_result ; 
   ];
.
   
:inconsistentResult a prov:Entity .

### An implicit process of analysing the intermediate result to confirm its expected consistency

analyse_intermediate_result
   a   prov:Activity ;
   prov:startedAtTime   "2011-07-15T12:52:02Z"^^xsd:dateTime;
   prov:endedAtTime     "2011-07-16T01:52:02Z"^^xsd:dateTime;
.
```

If this Activity prov:wasEndedBy Entity :e1, then it can qualify how it was ended using prov:qualifiedEnd [ a prov:End; prov:entity :e1; :foo :bar ].

has super-properties
:   * [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op

has domain
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

has range
:   * [prov:End](#End "http://www.w3.org/ns/prov#End")

qualifies
:   [prov:wasEndedBy](#wasEndedBy "http://www.w3.org/ns/prov#wasEndedBy") op

PROV-DM term
:   [End](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-End "prov-dm")

### (70) Property: [prov:qualifiedDelegation](#qualifiedDelegation) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#qualifiedDelegation

Delegation is the assignment of authority and responsibility to an agent (by itself or by another agent) to carry out a specific activity as a delegate or representative, while the agent it acts on behalf of retains some responsibility for the outcome of the delegated work.
For example, a student acted on behalf of his supervisor, who acted on behalf of the department chair, who acted on behalf of the university; all those agents are responsible in some way for the activity that took place but we do not say explicitly who bears responsibility and to what degree.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix ex:   <http://example.com/vocab#> .
@prefix :     <http://example.com/> .

:traffic-stop
   a prov:Activity;
   prov:wasAssociatedWith :chauffeur, 
                          :traffic-officer-34;
   prov:qualifiedAssociation [
      a prov:Association;
      prov:agent :chauffeur;
      
      # The chauffeur was the one violating traffic rules.
      prov:hadRole :violator; 
   ];
   prov:qualifiedAssociation [
      a prov:Association;
      prov:agent :traffic-officer-34;
      
      # The officer was the one enforcing the traffic rules.
      prov:hadRole :enforcer; 
   ];
.

:chauffeur       
   a prov:Person;
   prov:actedOnBehalfOf :celebrity-in-car;
   prov:qualifiedDelegation [
      a prov:Delegation;
      prov:agent :celebrity-in-car;
      
      # The celebrity employed the chauffeur during the enforcement.
      prov:hadRole :employer; 
      prov:hadActivity  :driving-during-the-year ;
   ];
.

#The chaffeur was employed for a whole year as a driver
:driving-during-the-year   a   prov:Activity ;
   prov:wasAssociatedWith :chaffeur;
   prov:qualifiedAssociation [
      a prov:Association;
      prov:hadRole :driver;
   ];
   prov:startedAtTime "2011-07-16T01:52:02Z"^^xsd:dateTime;
   prov:endedAtTime "2012-07-16T01:52:02Z"^^xsd:dateTime;;
.

:traffic_officer_34 
   a prov:Person;
   prov:actedOnBehalfOf :city-of-Paris;
   prov:qualifiedDelegation [
      a prov:Delegation;
      prov:agent :city-of-Paris;
      
      # The city of Paris employed the officer during the enforcement.
      prov:hadRole :employer; 
      prov:hadActivity      :control-city-traffic ;
   ];
.

:control-city-traffic   a   prov:Activity .
```

If this Agent prov:actedOnBehalfOf Agent :ag, then it can qualify how with prov:qualifiedResponsibility [ a prov:Responsibility; prov:agent :ag; :foo :bar ].

has super-properties
:   * [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") op

has domain
:   * [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent")

has range
:   * [prov:Delegation](#Delegation "http://www.w3.org/ns/prov#Delegation")

qualifies
:   [prov:actedOnBehalfOf](#actedOnBehalfOf "http://www.w3.org/ns/prov#actedOnBehalfOf") op

PROV-DM term
:   [delegation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-delegation "prov-dm")

### (71) Property: [prov:influencer](#influencer) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#influencer

This property is used as part of the qualified influence pattern. Subclasses of prov:Influence use these subproperties to reference the resource (Entity, Agent, or Activity) whose influence is being qualified.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:illustrationActivity 
   a prov:Activity;
   prov:qualifiedUsage         :illustration_usage;
   prov:qualifiedAssociation   :illustration_association;
   prov:qualifiedCommunication :illustration_communication;
.

:illustration_usage
   a prov:Usage;
   prov:entity     :aggregatedByRegions;
   prov:influencer :aggregatedByRegions; # Inferred
.

:illustration_association
   a prov:Association;
   prov:agent      :derek;
   prov:influencer :derek; # - - - - - - - Inferred
.

:illustration_communication
   a prov:Communication;
   prov:activity   :aggregationActivity;
   prov:influencer :aggregationActivity; # Inferred
.

:aggregationActivity a prov:Activity .  
:derek               a prov:Agent .
:aggregatedByRegions a prov:Entity .
```

Subproperties of prov:influencer are used to cite the object of an unqualified PROV-O triple whose predicate is a subproperty of prov:wasInfluencedBy (e.g. prov:used, prov:wasGeneratedBy). prov:influencer is used much like rdf:object is used.

has domain
:   * [prov:Influence](#Influence "http://www.w3.org/ns/prov#Influence")

has range


has sub-properties
:   * [prov:agent](#p_agent "http://www.w3.org/ns/prov#agent")
    * [prov:entity](#p_entity "http://www.w3.org/ns/prov#entity")
    * [prov:activity](#p_activity "http://www.w3.org/ns/prov#activity")

PROV-DM term
:   [influence](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-influence "prov-dm")

### (72) Property: [prov:entity](#p_entity) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#entity

The prov:entity property references an prov:Entity which influenced a resource. This property applies to an prov:EntityInfluence, which is given by a subproperty of prov:qualifiedInfluence from the influenced prov:Entity, prov:Activity or prov:Agent.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:world-literature-homework-submission-32 
   a prov:Entity;
   prov:hadPrimarySource :donQuixote;
   prov:qualifiedPrimarySource [
      a prov:PrimarySource;
      prov:entity :donQuixote;
      # Other attributes of the relationship
   ];
.

:donQuixote a prov:Entity .
```

has super-properties
:   * [prov:influencer](#influencer "http://www.w3.org/ns/prov#influencer") op

has domain
:   * [prov:EntityInfluence](#EntityInfluence "http://www.w3.org/ns/prov#EntityInfluence")

has range
:   * [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity")

### (73) Property: [prov:hadUsage](#hadUsage) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#hadUsage

Usage is the beginning of utilizing an entity by an activity. Before usage, the activity had not begun to utilize this entity and could not have been affected by the entity.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .
:digestedProteinSample1
   a prov:Entity;
   prov:wasDerivedFrom :proteinSample;
   prov:qualifiedDerivation [
      a prov:Derivation;
      prov:hadUsage [
         a prov:Usage;
         prov:entity :Trypsin;
        prov:hadRole :treatmentEnzyme;
      ];
   ];
   prov:entity :proteinSample;
.
:proteinSample a prov:Entity .
```

The \_optional\_ Usage involved in an Entity's Derivation.

has domain
:   * [prov:Derivation](#Derivation "http://www.w3.org/ns/prov#Derivation")

has range
:   * [prov:Usage](#Usage "http://www.w3.org/ns/prov#Usage")

PROV-DM term
:   [Usage](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Usage "prov-dm")

### (74) Property: [prov:hadGeneration](#hadGeneration) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#hadGeneration

Generation is the completion of production of a new entity by an activity. This entity did not exist before generation and becomes available for usage after this generation.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix ex:   <http://example.com/vocab#> .
@prefix :     <http://example.com/> .

:bar_chart
   a prov:Entity, ex:Chart;
   prov:wasDerivedFrom   :aggregatedByRegions;
   prov:qualifiedDerivation [
      a prov:Derivation;
      prov:entity        :aggregatedByRegions;
      prov:hadGeneration :illustration;
   ];
.

:aggregatedByRegions a ex:Dataset .

:illustration
   a prov:Generation,
     prov:InstantaneousEvent;
   prov:activity :illustrationActivity;
   prov:atTime   "2012-04-03T00:00:11Z"^^xsd:dateTime;
.

:illustrationActivity
   a prov:Activity;
   prov:startedAtTime "2012-04-03T00:00:00Z"^^xsd:dateTime;
   prov:endedAtTime   "2012-04-03T00:00:25Z"^^xsd:dateTime;
.
```

The \_optional\_ Generation involved in an Entity's Derivation.

has domain
:   * [prov:Derivation](#Derivation "http://www.w3.org/ns/prov#Derivation")

has range
:   * [prov:Generation](#Generation "http://www.w3.org/ns/prov#Generation")

PROV-DM term
:   [Generation](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Generation "prov-dm")

### (75) Property: [prov:activity](#p_activity) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#activity

The prov:activity property references an prov:Activity which influenced a resource. This property applies to an prov:ActivityInfluence, which is given by a subproperty of prov:qualifiedInfluence from the influenced prov:Entity, prov:Activity or prov:Agent.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:making-bar-chart
   a prov:Generation, 
     prov:ActivityInfluence;
   prov:activity :illustrating;
   rdfs:comment "Ended up with bar chart as line chart looked ugly."@en;
.

:illustrating a prov:Activity .

:bar_chart 
   a prov:Entity;
   prov:wasGeneratedBy :illustrating;
   prov:qualifiedGeneration :making-bar-chart;
.
```

has super-properties
:   * [prov:influencer](#influencer "http://www.w3.org/ns/prov#influencer") op

has domain
:   * [prov:ActivityInfluence](#ActivityInfluence "http://www.w3.org/ns/prov#ActivityInfluence")

has range
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

### (76) Property: [prov:agent](#p_agent) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#agent

The prov:agent property references an prov:Agent which influenced a resource. This property applies to an prov:AgentInfluence, which is given by a subproperty of prov:qualifiedInfluence from the influenced prov:Entity, prov:Activity or prov:Agent.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:world_flight_1937
   a prov:Activity;

   prov:wasAssociatedWith <http://dbpedia.org/resource/Amelia_Earhart>,
                          <http://dbpedia.org/resource/Purdue_University>,
                          <http://dbpedia.org/resource/Lockheed_Aircraft_Company>;
   prov:qualifiedAssociation [
      a prov:Association;
      prov:agent   <http://dbpedia.org/resource/Amelia_Earhart>;
      prov:hadRole :pilot;
   ];
   prov:qualifiedAssociation [
      a prov:Association;
      prov:agent   <http://dbpedia.org/resource/Purdue_University>;
      prov:hadRole :financer;
   ];
   prov:qualifiedAssociation [
      a prov:Association;
      prov:agent   <http://dbpedia.org/resource/Lockheed_Aircraft_Company>;
      prov:hadRole :plane_builder;
   ];

   rdfs:seeAlso <http://en.wikipedia.org/wiki/Amelia_Earhart#1937_world_flight>;
.

<http://dbpedia.org/resource/Amelia_Earhart>            a prov:Person,       prov:Agent .
<http://dbpedia.org/resource/Purdue_University>         a prov:Organization, prov:Agent .
<http://dbpedia.org/resource/Lockheed_Aircraft_Company> a prov:Organization, prov:Agent .
```

has super-properties
:   * [prov:influencer](#influencer "http://www.w3.org/ns/prov#influencer") op

has domain
:   * [prov:AgentInfluence](#AgentInfluence "http://www.w3.org/ns/prov#AgentInfluence")

has range
:   * [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent")

### (77) Property: [prov:hadPlan](#hadPlan) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#hadPlan

A plan is an entity that represents a set of actions or steps intended by one or more agents to achieve some goals.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:illustrating
   a prov:Activity; 
   prov:wasAssociatedWith :derek, 
                          :steve;
   prov:qualifiedAssociation [
      a prov:Association;
      prov:agent   :steve;
      prov:hadRole :stylist;
      prov:hadPlan :style-guide;
      rdfs:comment "Steve followed the style guide"@en;
   ];
.

:style-guide
   a prov:Plan, prov:Entity;
   rdfs:comment "Use blue graphs for positive spin, red for negative"@en;
.
```

The \_optional\_ Plan adopted by an Agent in Association with some Activity. Plan specifications are out of the scope of this specification.

has domain
:   * [prov:Association](#Association "http://www.w3.org/ns/prov#Association")

has range
:   * [prov:Plan](#Plan "http://www.w3.org/ns/prov#Plan")

PROV-DM term
:   [Association](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Association "prov-dm")

### (78) Property: [prov:hadActivity](#hadActivity) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#hadActivity

An activity is something that occurs over a period of time and acts upon or with entities; it may include consuming, processing, transforming, modifying, relocating, using, or generating entities.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

# The activity that which used, generated, invalidated 
# or was responsible for the entity. In this qualified Derivation 
# prov:hadActivity references the activity that generated the bar chart.

:bar_chart
   a prov:Entity;
   prov:wasDerivedFrom :aggregatedByRegions;
   prov:wasGeneratedBy :make_bar_chart;
   prov:qualifiedDerivation [
      a prov:Derivation;
      prov:entity      :aggregatedByRegions;
      prov:hadActivity :make_bar_chart; # references same activity as prov:wasGeneratedBy
   ];
.

:aggregatedByRegions a prov:Entity .
:make_bar_chart      a prov:Activity .
```

The \_optional\_ Activity of an Influence, which used, generated, invalidated, or was the responsibility of some Entity. This property is \_not\_ used by ActivityInfluence (use prov:activity instead).

This property has multiple RDFS domains to suit multiple OWL Profiles. See [PROV-O OWL Profile](#owl-profile).

has domain
:   * [prov:Delegation](#Delegation "http://www.w3.org/ns/prov#Delegation")
      or [prov:Derivation](#Derivation "http://www.w3.org/ns/prov#Derivation")
      or [prov:End](#End "http://www.w3.org/ns/prov#End")
      or [prov:Start](#Start "http://www.w3.org/ns/prov#Start")
    * [prov:Influence](#Influence "http://www.w3.org/ns/prov#Influence")

has range
:   * [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity")

PROV-DM term
:   [Activity](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-Activity "prov-dm")

### (79) Property: [prov:atTime](#atTime) dp back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#atTime

The PROV data model is implicitly based on a notion of instantaneous events (or just events), that mark transitions in the world. Events include generation, usage, or invalidation of entities, as well as starting or ending of activities. This notion of event is not first-class in the data model, but it is useful for explaining its other concepts and its semantics.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:Timearticle20120430_publication
   a prov:InstantaneousEvent;
   prov:atTime "2012-04-30T20:40:40"^^xsd:dateTime;
.
```

The time at which an InstantaneousEvent occurred, in the form of xsd:dateTime.

has domain
:   * [prov:InstantaneousEvent](#InstantaneousEvent "http://www.w3.org/ns/prov#InstantaneousEvent")

has range
:   * http://www.w3.org/2001/XMLSchema#dateTime

qualifies
:   [prov:invalidatedAtTime](#invalidatedAtTime "http://www.w3.org/ns/prov#invalidatedAtTime") dp

### (80) Property: [prov:hadRole](#hadRole) op back to [qualified properties](#prov-qualified-owl-terms-at-a-glance)

**IRI:**http://www.w3.org/ns/prov#hadRole

A role is the function of an entity or agent with respect to an activity, in the context of a usage, generation, invalidation, association, start, and end.

**Example**

```
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .
@prefix owl:  <http://www.w3.org/2002/07/owl#> .
@prefix prov: <http://www.w3.org/ns/prov#> .
@prefix :     <http://example.com/> .

:divideActivity 
   a prov:Activity;   

   prov:used         :variableA;
   prov:qualifiedUsage [
      a prov:Usage;
      prov:entity    :variableA;
      prov:hadRole   :dividend;          
   ];
   
   prov:used         :variableB;
   prov:qualifiedUsage [
      a prov:Usage;
      prov:entity    :variableB;
      prov:hadRole   :divisor;          
   ];
.
   
:variableA a prov:Entity .
:variableB a prov:Entity .

:dividend a prov:Role .
:divisor  a prov:Role .
```

The \_optional\_ Role that an Entity assumed in the context of an Activity. For example, :baking prov:used :spoon; prov:qualified [ a prov:Usage; prov:entity :spoon; prov:hadRole roles:mixing\_implement ].

This property has multiple RDFS domains to suit multiple OWL Profiles. See [PROV-O OWL Profile](#owl-profile).

has domain
:   * [prov:Association](#Association "http://www.w3.org/ns/prov#Association")
      or [prov:InstantaneousEvent](#InstantaneousEvent "http://www.w3.org/ns/prov#InstantaneousEvent")
    * [prov:Influence](#Influence "http://www.w3.org/ns/prov#Influence")

has range
:   * [prov:Role](#Role "http://www.w3.org/ns/prov#Role")

PROV-DM term
:   [attribute-role](https://www.w3.org/TR/2013/REC-prov-dm-20130430/Overview.html#term-attribute-role "prov-dm")

### 4.4 Term Index

The PROV-O terms in this cross reference are shown below alphabetically, along with their entry number.

[Table 4](#cross-reference-index): Term Index.

| PROV-O Term | Position within Cross Reference |
| --- | --- |
| [actedOnBehalfOf](#actedOnBehalfOf) | [Entry 12](#actedOnBehalfOf) |
| [Activity](#Activity) | [Entry 2](#Activity) |
| [activity](#p_activity) | [Entry 75](#p_activity) |
| [ActivityInfluence](#ActivityInfluence) | [Entry 45](#ActivityInfluence) |
| [Agent](#Agent) | [Entry 3](#Agent) |
| [agent](#p_agent) | [Entry 76](#p_agent) |
| [AgentInfluence](#AgentInfluence) | [Entry 49](#AgentInfluence) |
| [alternateOf](#alternateOf) | [Entry 20](#alternateOf) |
| [Association](#Association) | [Entry 51](#Association) |
| [atLocation](#atLocation) | [Entry 34](#atLocation) |
| [atTime](#atTime) | [Entry 79](#atTime) |
| [Attribution](#Attribution) | [Entry 50](#Attribution) |
| [Bundle](#Bundle) | [Entry 15](#Bundle) |
| [Collection](#Collection) | [Entry 13](#Collection) |
| [Communication](#Communication) | [Entry 47](#Communication) |
| [Delegation](#Delegation) | [Entry 53](#Delegation) |
| [Derivation](#Derivation) | [Entry 41](#Derivation) |
| [EmptyCollection](#EmptyCollection) | [Entry 14](#EmptyCollection) |
| [End](#End) | [Entry 40](#End) |
| [endedAtTime](#endedAtTime) | [Entry 10](#endedAtTime) |
| [Entity](#Entity) | [Entry 1](#Entity) |
| [entity](#p_entity) | [Entry 72](#p_entity) |
| [EntityInfluence](#EntityInfluence) | [Entry 37](#EntityInfluence) |
| [generated](#generated) | [Entry 35](#generated) |
| [generatedAtTime](#generatedAtTime) | [Entry 22](#generatedAtTime) |
| [Generation](#Generation) | [Entry 46](#Generation) |
| [hadActivity](#hadActivity) | [Entry 78](#hadActivity) |
| [hadGeneration](#hadGeneration) | [Entry 74](#hadGeneration) |
| [hadMember](#hadMember) | [Entry 29](#hadMember) |
| [hadPlan](#hadPlan) | [Entry 77](#hadPlan) |
| [hadPrimarySource](#hadPrimarySource) | [Entry 23](#hadPrimarySource) |
| [hadRole](#hadRole) | [Entry 80](#hadRole) |
| [hadUsage](#hadUsage) | [Entry 73](#hadUsage) |
| [Influence](#Influence) | [Entry 36](#Influence) |
| [influenced](#influenced) | [Entry 33](#influenced) |
| [influencer](#influencer) | [Entry 71](#influencer) |
| [InstantaneousEvent](#InstantaneousEvent) | [Entry 54](#InstantaneousEvent) |
| [invalidated](#invalidated) | [Entry 32](#invalidated) |
| [invalidatedAtTime](#invalidatedAtTime) | [Entry 27](#invalidatedAtTime) |
| [Invalidation](#Invalidation) | [Entry 48](#Invalidation) |
| [Location](#Location) | [Entry 19](#Location) |
| [Organization](#Organization) | [Entry 18](#Organization) |
| [Person](#Person) | [Entry 16](#Person) |
| [Plan](#Plan) | [Entry 52](#Plan) |
| [PrimarySource](#PrimarySource) | [Entry 42](#PrimarySource) |
| [qualifiedAssociation](#qualifiedAssociation) | [Entry 68](#qualifiedAssociation) |
| [qualifiedAttribution](#qualifiedAttribution) | [Entry 63](#qualifiedAttribution) |
| [qualifiedCommunication](#qualifiedCommunication) | [Entry 67](#qualifiedCommunication) |
| [qualifiedDelegation](#qualifiedDelegation) | [Entry 70](#qualifiedDelegation) |
| [qualifiedDerivation](#qualifiedDerivation) | [Entry 59](#qualifiedDerivation) |
| [qualifiedEnd](#qualifiedEnd) | [Entry 69](#qualifiedEnd) |
| [qualifiedGeneration](#qualifiedGeneration) | [Entry 58](#qualifiedGeneration) |
| [qualifiedInfluence](#qualifiedInfluence) | [Entry 57](#qualifiedInfluence) |
| [qualifiedInvalidation](#qualifiedInvalidation) | [Entry 64](#qualifiedInvalidation) |
| [qualifiedPrimarySource](#qualifiedPrimarySource) | [Entry 60](#qualifiedPrimarySource) |
| [qualifiedQuotation](#qualifiedQuotation) | [Entry 61](#qualifiedQuotation) |
| [qualifiedRevision](#qualifiedRevision) | [Entry 62](#qualifiedRevision) |
| [qualifiedStart](#qualifiedStart) | [Entry 65](#qualifiedStart) |
| [qualifiedUsage](#qualifiedUsage) | [Entry 66](#qualifiedUsage) |
| [Quotation](#Quotation) | [Entry 43](#Quotation) |
| [Revision](#Revision) | [Entry 44](#Revision) |
| [Role](#Role) | [Entry 55](#Role) |
| [SoftwareAgent](#SoftwareAgent) | [Entry 17](#SoftwareAgent) |
| [specializationOf](#specializationOf) | [Entry 21](#specializationOf) |
| [Start](#Start) | [Entry 39](#Start) |
| [startedAtTime](#startedAtTime) | [Entry 7](#startedAtTime) |
| [Usage](#Usage) | [Entry 38](#Usage) |
| [used](#used) | [Entry 8](#used) |
| [value](#value) | [Entry 24](#value) |
| [wasAssociatedWith](#wasAssociatedWith) | [Entry 11](#wasAssociatedWith) |
| [wasAttributedTo](#wasAttributedTo) | [Entry 6](#wasAttributedTo) |
| [wasDerivedFrom](#wasDerivedFrom) | [Entry 5](#wasDerivedFrom) |
| [wasEndedBy](#wasEndedBy) | [Entry 31](#wasEndedBy) |
| [wasGeneratedBy](#wasGeneratedBy) | [Entry 4](#wasGeneratedBy) |
| [wasInfluencedBy](#wasInfluencedBy) | [Entry 56](#wasInfluencedBy) |
| [wasInformedBy](#wasInformedBy) | [Entry 9](#wasInformedBy) |
| [wasInvalidatedBy](#wasInvalidatedBy) | [Entry 28](#wasInvalidatedBy) |
| [wasQuotedFrom](#wasQuotedFrom) | [Entry 25](#wasQuotedFrom) |
| [wasRevisionOf](#wasRevisionOf) | [Entry 26](#wasRevisionOf) |
| [wasStartedBy](#wasStartedBy) | [Entry 30](#wasStartedBy) |

A. PROV-O OWL Profile
---------------------

*This section is non-normative.*

To encourage widespread adoption, PROV-O's design is intentionally minimal and lightweight.
Because the OWL 2 RL profile is aimed at RDF applications that require scalable reasoning without sacrificing too much expressive power [[OWL2-PRIMER](#bib-OWL2-PRIMER)],
it served as a baseline for all axioms included in PROV-O. The PROV-O axioms that do not suit the OWL 2 RL profile are listed in
[Table 5](#owl-rl-violations).
All five use an anonymous *class union* for the domain or range of a property, while OWL 2 RL requires the classes to be explicitly named.
Although introducing "placeholder" classes would have suited the
OWL 2 RL profile, these additional "abstract" classes would have been irrelevant
to the modeling of provenance information, increased the size of PROV-O unnecessarily, and exposed a potential to confuse users.
All five axioms listed in the following table *use a non-superclass expression in a position that requires a superclass expression*
and do not conform to the OWL 2 RL Profile.

[Table 5](#owl-rl-violations): All OWL Axioms in PROV-O that do not conform to the OWL-RL profile.

| Non OWL-RL PROV-O Axiom |
| --- |
| [prov:atLocation](#atLocation) rdfs:domain [ owl:unionOf (prov:Activity prov:Agent prov:Entity prov:InstantaneousEvent) ] |
| [prov:wasInfluencedBy](#wasInfluencedBy) rdfs:domain [ owl:unionOf (prov:Activity prov:Agent prov:Entity) ] |
| [prov:wasInfluencedBy](#wasInfluencedBy) rdfs:range [ owl:unionOf (prov:Activity prov:Agent prov:Entity) ] |
| [prov:hadActivity](#hadActivity) rdfs:domain [ owl:unionOf (prov:Delegation prov:Derivation prov:Start prov:End) ] |
| [prov:hadRole](#hadRole) rdfs:domain [ owl:unionOf (prov:Association prov:InstantaneousEvent) ] |

To provide guidance for OWL 2 RL environments that ignore the union domain axioms, some
property domains or ranges have also been defined with the closest
common superclass for the classes in the union, as shown in the following table.

[Table 6](#owl-rl-extra-domains): Intersecting OWL2 RL compatible domains/ranges

| Property | Direction | Domain/range |
| --- | --- | --- |
| [prov:atLocation](#atLocation) | rdfs:domain | (implied: owl:Thing) |
| [prov:wasInfluencedBy](#wasInfluencedBy) | rdfs:domain / rdfs:range | (implied: owl:Thing) |
| [prov:hadActivity](#hadActivity) | rdfs:domain | [prov:Influence](#Influence) |
| [prov:hadRole](#hadRole) | rdfs:domain | [prov:Influence](#Influence) |

Multiple RDFS domains and ranges [[RDF-SCHEMA](#bib-RDF-SCHEMA)] for a property are
[interpreted as an intersection](https://www.w3.org/TR/2004/REC-rdf-schema-20040210/#ch_range),
and thus the above do not provide any additional
information in an OWL 2 DL or OWL 2 Full profile, which also understands
the unions. The more general domain should not be
interpreted as saying, e.g.,
"prov:hadActivity can be used with any prov:Influence",
but as
"Anything using prov:hadActivity is (at least) a prov:Influence".

B. Names of inverse properties
------------------------------

To maximize interoperability, PROV-O intentionally avoids defining too many properties' inverses.
In fact, it only defines two ([prov:generated](#generated) and [prov:invalidated](#invalidated)).
When all inverses are defined for all properties, modelers may choose from two logically equivalent properties when making each assertion.
Although the two options may be logically equivalent, developers consuming the assertions may need to exert extra effort to handle both
(e.g., by either adding an OWL reasoner or writing code and queries to handle both cases).
This extra effort can be reduced by preferring one inverse over another.

For example, the first PROV-O statement (below) could just as easily be asserted as the second statement.
But if a client queries using [prov:wasDerivedFrom](#wasDerivedFrom) when :hadDerivation
was used in the assertion, no results will be returned unless OWL reasoning is applied (or the size of the query is doubled).

```
<http://www.w3.org/TR/prov-o/>  prov:wasDerivedFrom <http://www.w3.org/TR/prov-dm/> .

   # These two statements are equivalent if prov:wasDerivedFrom is an inverse of :hadDerivation.
   # But extra effort is required to handle both cases (if one is not already using OWL reasoning).
   # We cannot assume that everybody is using OWL reasoning.
   # We do not want people to write more code and query than necessary.

<http://www.w3.org/TR/prov-dm/>     :hadDerivation  <http://www.w3.org/TR/prov-o/>  .
```

So, PROV-O avoids this situation by encouraging modelers to use one property instead of its inverse; the preferred property to use is the one defined in the PROV-O ontology. Those asserting and querying for the preferred property avoid the need for OWL reasoning, additional code, and larger queries while maintaining the same level of interoperability.

However, the **absence** of defined inverses can lead to a **different** risk to interoperability.
Because modelers are free to create their own properties to suit their needs,
they may be motivated to assert the inverse of any PROV-O property defined herein.

For example, since PROV-O does not define the inverse of [prov:wasDerivedFrom](#wasDerivedFrom),
and if three developers would rather model their assertions in the opposite direction, the following set of assertions might be found in the future web of provenance.
These assertions are not in an interoperable form without the use of an OWL reasoner, additional code, or larger queries.

```
# If PROV-O's properties' inverses are not defined, modelers may be motivated to introduce their own inverse property name.
# The following three statements are equivalent if their predicates are all inverses of prov:wasDerivedFrom.

<http://www.w3.org/TR/prov-dm/>    my:hadDerivation  <http://www.w3.org/TR/prov-o/>  .
<http://www.w3.org/TR/prov-dm/>  your:ledTo          <http://www.w3.org/TR/prov-o/>  .
<http://www.w3.org/TR/prov-dm/> their:derivedTo      <http://www.w3.org/TR/prov-o/>  .
```

To balance these two interoperability risks, this document reserves the names of the PROV-O inverses.
The name of a property's inverse is determined by appending the value of its http://www.w3.org/ns/prov#inverse
annotation to the PROV namespace (http://www.w3.org/ns/prov#).
Modelers wishing to use inverses of the properties defined by PROV-O *should* use those reserved by this document.

For example, the same three modelers above that defined `my:hadDerivation`, `your:ledTo`, and `their:derivedTo` should instead look for the http://www.w3.org/ns/prov#inverse annotation on [prov:wasDerivedFrom](#wasDerivedFrom) to determine that they should use the property http://www.w3.org/ns/prov#hadDerivation.

```
@prefix prov: <http://www.w3.org/ns/prov#> .

# Each PROV-O property is annotated with the local name of its inverse.

prov:wasDerivedFrom
   a owl:ObjectProperty;
   rdfs:isDefinedBy <http://www.w3.org/ns/prov#>;
   prov:inverse     "hadDerivation";
   rdfs:domain  prov:Entity;
   rdfs:range   prov:Entity;
.

# Instead of defining their own, modelers should use the
# recommended inverse local name within the PROV namespace:

<http://www.w3.org/TR/prov-dm/> prov:hadDerivation <http://www.w3.org/TR/prov-o/>  .

# Following this recommendation avoids a proliferation of inverse definitions, 
# while encouraging the use of one inverse over another.
# This increases interoperability.
```

The following table lists the recommended inverse names that should be used if a modeler does not want to use the recommended PROV-O property.
For convenience, [this file](https://www.w3.org/ns/prov-o-inverses) lists the resulting inverse properties.

[Table 5](#inverse-names-table): Names of inverses

| Domain | PROV-O Property | Recommended inverse name | Range |
| [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent") | [prov:actedOnBehalfOf](#actedOnBehalfOf "http://www.w3.org/ns/prov#actedOnBehalfOf") | prov:hadDelegate | [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent") |
| [prov:ActivityInfluence](#ActivityInfluence "http://www.w3.org/ns/prov#ActivityInfluence") | [prov:activity](#p_activity "http://www.w3.org/ns/prov#activity") | prov:activityOfInfluence | [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") |
| [prov:AgentInfluence](#AgentInfluence "http://www.w3.org/ns/prov#AgentInfluence") | [prov:agent](#p_agent "http://www.w3.org/ns/prov#agent") | prov:agentOfInfluence | [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:alternateOf](#alternateOf "http://www.w3.org/ns/prov#alternateOf") | prov:alternateOf | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") |
| union | [prov:atLocation](#atLocation "http://www.w3.org/ns/prov#atLocation") | prov:locationOf | [prov:Location](#Location "http://www.w3.org/ns/prov#Location") |
| [prov:EntityInfluence](#EntityInfluence "http://www.w3.org/ns/prov#EntityInfluence") | [prov:entity](#p_entity "http://www.w3.org/ns/prov#entity") | prov:entityOfInfluence | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") |
| [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:generated](#generated "http://www.w3.org/ns/prov#generated") | prov:wasGeneratedBy | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") |
| union | [prov:hadActivity](#hadActivity "http://www.w3.org/ns/prov#hadActivity") | prov:wasActivityOfInfluence | [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") |
| [prov:Derivation](#Derivation "http://www.w3.org/ns/prov#Derivation") | [prov:hadGeneration](#hadGeneration "http://www.w3.org/ns/prov#hadGeneration") | prov:generatedAsDerivation | [prov:Generation](#Generation "http://www.w3.org/ns/prov#Generation") |
| [prov:Collection](#Collection "http://www.w3.org/ns/prov#Collection") | [prov:hadMember](#hadMember "http://www.w3.org/ns/prov#hadMember") | prov:wasMemberOf | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") |
| [prov:Association](#Association "http://www.w3.org/ns/prov#Association") | [prov:hadPlan](#hadPlan "http://www.w3.org/ns/prov#hadPlan") | prov:wasPlanOf | [prov:Plan](#Plan "http://www.w3.org/ns/prov#Plan") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:hadPrimarySource](#hadPrimarySource "http://www.w3.org/ns/prov#hadPrimarySource") | prov:wasPrimarySourceOf | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") |
| union | [prov:hadRole](#hadRole "http://www.w3.org/ns/prov#hadRole") | prov:wasRoleIn | [prov:Role](#Role "http://www.w3.org/ns/prov#Role") |
| [prov:Derivation](#Derivation "http://www.w3.org/ns/prov#Derivation") | [prov:hadUsage](#hadUsage "http://www.w3.org/ns/prov#hadUsage") | prov:wasUsedInDerivation | [prov:Usage](#Usage "http://www.w3.org/ns/prov#Usage") |
|  | [prov:influenced](#influenced "http://www.w3.org/ns/prov#influenced") | prov:wasInfluencedBy |
| [prov:Influence](#Influence "http://www.w3.org/ns/prov#Influence") | [prov:influencer](#influencer "http://www.w3.org/ns/prov#influencer") | prov:hadInfluence | union |
| [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:invalidated](#invalidated "http://www.w3.org/ns/prov#invalidated") | prov:wasInvalidatedBy | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") |
| [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:qualifiedAssociation](#qualifiedAssociation "http://www.w3.org/ns/prov#qualifiedAssociation") | prov:qualifiedAssociationOf | [prov:Association](#Association "http://www.w3.org/ns/prov#Association") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:qualifiedAttribution](#qualifiedAttribution "http://www.w3.org/ns/prov#qualifiedAttribution") | prov:qualifiedAttributionOf | [prov:Attribution](#Attribution "http://www.w3.org/ns/prov#Attribution") |
| [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:qualifiedCommunication](#qualifiedCommunication "http://www.w3.org/ns/prov#qualifiedCommunication") | prov:qualifiedCommunicationOf | [prov:Communication](#Communication "http://www.w3.org/ns/prov#Communication") |
| [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent") | [prov:qualifiedDelegation](#qualifiedDelegation "http://www.w3.org/ns/prov#qualifiedDelegation") | prov:qualifiedDelegationOf | [prov:Delegation](#Delegation "http://www.w3.org/ns/prov#Delegation") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:qualifiedDerivation](#qualifiedDerivation "http://www.w3.org/ns/prov#qualifiedDerivation") | prov:qualifiedDerivationOf | [prov:Derivation](#Derivation "http://www.w3.org/ns/prov#Derivation") |
| [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:qualifiedEnd](#qualifiedEnd "http://www.w3.org/ns/prov#qualifiedEnd") | prov:qualifiedEndOf | [prov:End](#End "http://www.w3.org/ns/prov#End") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:qualifiedGeneration](#qualifiedGeneration "http://www.w3.org/ns/prov#qualifiedGeneration") | prov:qualifiedGenerationOf | [prov:Generation](#Generation "http://www.w3.org/ns/prov#Generation") |
| union | [prov:qualifiedInfluence](#qualifiedInfluence "http://www.w3.org/ns/prov#qualifiedInfluence") | prov:qualifiedInfluenceOf | [prov:Influence](#Influence "http://www.w3.org/ns/prov#Influence") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:qualifiedInvalidation](#qualifiedInvalidation "http://www.w3.org/ns/prov#qualifiedInvalidation") | prov:qualifiedInvalidationOf | [prov:Invalidation](#Invalidation "http://www.w3.org/ns/prov#Invalidation") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:qualifiedPrimarySource](#qualifiedPrimarySource "http://www.w3.org/ns/prov#qualifiedPrimarySource") | prov:qualifiedSourceOf | [prov:PrimarySource](#PrimarySource "http://www.w3.org/ns/prov#PrimarySource") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:qualifiedQuotation](#qualifiedQuotation "http://www.w3.org/ns/prov#qualifiedQuotation") | prov:qualifiedQuotationOf | [prov:Quotation](#Quotation "http://www.w3.org/ns/prov#Quotation") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:qualifiedRevision](#qualifiedRevision "http://www.w3.org/ns/prov#qualifiedRevision") | prov:revisedEntity | [prov:Revision](#Revision "http://www.w3.org/ns/prov#Revision") |
| [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:qualifiedStart](#qualifiedStart "http://www.w3.org/ns/prov#qualifiedStart") | prov:qualifiedStartOf | [prov:Start](#Start "http://www.w3.org/ns/prov#Start") |
| [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:qualifiedUsage](#qualifiedUsage "http://www.w3.org/ns/prov#qualifiedUsage") | prov:qualifiedUsingActivity | [prov:Usage](#Usage "http://www.w3.org/ns/prov#Usage") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:specializationOf](#specializationOf "http://www.w3.org/ns/prov#specializationOf") | prov:generalizationOf | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") |
| [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:used](#used "http://www.w3.org/ns/prov#used") | prov:wasUsedBy | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") |
| [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:wasAssociatedWith](#wasAssociatedWith "http://www.w3.org/ns/prov#wasAssociatedWith") | prov:wasAssociateFor | [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:wasAttributedTo](#wasAttributedTo "http://www.w3.org/ns/prov#wasAttributedTo") | prov:contributed | [prov:Agent](#Agent "http://www.w3.org/ns/prov#Agent") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:wasDerivedFrom](#wasDerivedFrom "http://www.w3.org/ns/prov#wasDerivedFrom") | prov:hadDerivation | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") |
| [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:wasEndedBy](#wasEndedBy "http://www.w3.org/ns/prov#wasEndedBy") | prov:ended | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:wasGeneratedBy](#wasGeneratedBy "http://www.w3.org/ns/prov#wasGeneratedBy") | prov:generated | [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") |
| union | [prov:wasInfluencedBy](#wasInfluencedBy "http://www.w3.org/ns/prov#wasInfluencedBy") | prov:influenced | union |
| [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:wasInformedBy](#wasInformedBy "http://www.w3.org/ns/prov#wasInformedBy") | prov:informed | [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:wasInvalidatedBy](#wasInvalidatedBy "http://www.w3.org/ns/prov#wasInvalidatedBy") | prov:invalidated | [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:wasQuotedFrom](#wasQuotedFrom "http://www.w3.org/ns/prov#wasQuotedFrom") | prov:quotedAs | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") |
| [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") | [prov:wasRevisionOf](#wasRevisionOf "http://www.w3.org/ns/prov#wasRevisionOf") | prov:hadRevision | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") |
| [prov:Activity](#Activity "http://www.w3.org/ns/prov#Activity") | [prov:wasStartedBy](#wasStartedBy "http://www.w3.org/ns/prov#wasStartedBy") | prov:started | [prov:Entity](#Entity "http://www.w3.org/ns/prov#Entity") |

C. Changes since WD-prov-o-20120724
-----------------------------------

*This section is non-normative.*

* Restated [prov:hadRole](#hadRole)'s domain to 'Association or InstantaneousEvent'
  instead of the original that enumerated the subclasses of InstantaneousEvent
  ('Association or End or Generation or Invalidation or Start or Usage').
* Renamed prov:Source to [prov:PrimarySource](#PrimarySource)
  and prov:qualifiedSource to [prov:qualifiedPrimarySource](#qualifiedPrimarySource).
* Examples have been rewritten to avoid usage of TriG named
  graph syntax except for when showing bundles in
  prov:asInBundle and prov:mentionOf (since removed to a separate Note). A citation to TriG was added.
* Some examples have been elaborated to use resource names
  like `:illustration_usage` rather than
  `:usage_1`.
* Fixed naming mismatch by changing prov:hadOriginalSource to [prov:hadPrimarySource](#hadPrimarySource).
* Rephrased definitions for [prov:EntityInfluence](#EntityInfluence),
  [prov:ActivityInfluence](#ActivityInfluence), and [prov:AgentInfluence](#AgentInfluence)
  to align with the definition of their superclass [prov:Influence](#Influence).
* Updated definitions for [prov:Start](#Start) and
  [prov:End](#End) from PROV-DM.
* The property chain for [prov:wasInformedBy](#wasInformedBy) was fixed from
  "qualifiedCommunication o entity subproperty of wasInformedBy"
  to "[prov:qualifiedCommunication](#qualifiedCommunication) o [prov:activity](#p_activity) subproperty of wasInformedBy"
* Removed prov:mentionOf and prov:asInBundle, which have been relocated to its own Note.
* Added comments encouraging the use of the more specific forms of [prov:Influence](#Influence).
* Added uniform references to other "dated" PROV documents.
* Added [prefix namespace table](#namespace-table).
* Added [Compliance with this document](#compliance) section.
* Corrected Turtle syntax for RL violations in [PROV-O OWL Profile](#owl-profile) section. They were missing owl:unionOf.
* Updated attributions for the tools used to produce this document in [Acknowledgements](#acknowledgements) section.
* Reworked the [Expanded Terms](#description-expanded-terms) narrative and examples to better highlight each term.

D. Changes since CR-prov-o-20121211
-----------------------------------

*This section is non-normative.*

* Updated exemplar in cross reference entry [prov:hadGeneration](#hadGeneration) to include [prov:activity](#p_activity) property.
* Reordered class and predicate terms from alphabetical to a more natural narrative-based order.
* Added [Term Index](#cross-reference-index) to aid reading this document in printed form.
* Fixed typo 'iteself' to 'itself'.
* Removed inaccurate property characteristics (AsymmetricProperty, IrreflexiveProperty)
  in third example of [Appendix B](#inverse-names).
* Added note to [Starting Point Terms](#description-starting-point-terms) stating that
  rdf:type and rdfs:label are used to express PROV-DM's prov:type and prov:label.
* Updated prov:value's out-of-date definition to conform to PROV-DM's (i.e., "Provides a value that is a direct representation of an entity.".
* Updated prov:wasDerivedFrom's out-of-date definition to conform to PROV-DM's
  (i.e., "A derivation is a transformation of an entity into another, an update of an entity resulting in a new one, or the construction of a new entity based on a pre-existing entity.".
* Added xsd:dateType datatypes to exemplar in Invalidation and invalidatedAtTime.
* Fixed some incorrect wasAttributedTo/wasAssociatedWith in the cross reference exemplars.
* Changed the status of this document section: added new documents to the PROV Family of Document, and removed the how to read section, referring instead to PROV-OVERVIEW.
* Changed all URLs to PROV documents.

E. Changes since PR-prov-o-20130312
-----------------------------------

*This section is non-normative.*

* Fixed typo in [alternateOf](https://www.w3.org/TR/2013/PR-prov-o-20130312/#alternateOf) example: :london\_forecast\_043 became :london\_forecast\_0413
* Changed capitalization in definitions for
  [Organization](https://www.w3.org/TR/2013/PR-prov-o-20130312/#SoftwareAgent) ([new](#Organization)) and
  [Person](https://www.w3.org/TR/2013/PR-prov-o-20130312/#SoftwareAgent) ([new](#Person)).

F. Acknowledgements
-------------------

*This section is non-normative.*

This document has been produced by the PROV Working Group, and its contents reflect extensive discussion within the Working Group as a whole. The editors extend special thanks to Sandro Hawke (W3C/MIT) and Ivan Herman (W3C/ERCIM), W3C contacts for the PROV Working Group.

The editors acknowledge valuable contributions from the following:
Tom Baker,
David Booth,
Robert Freimuth,
Satrajit Ghosh,
Ralph Hodgson,
Renato Iannella,
Jacek Kopecky,
James Leigh,
Jacco van Ossenbruggen,
Alan Ruttenberg,
Reza Samavi, and
Antoine Zimmermann.

Members of the PROV Working Group at the time of publication of this document were:
Ilkay Altintas (Invited expert),
Reza B'Far (Oracle Corporation),
Khalid Belhajjame (University of Manchester),
James Cheney (University of Edinburgh, School of Informatics),
Sam Coppens (iMinds - Ghent University),
David Corsar (University of Aberdeen, Computing Science),
Stephen Cresswell (The National Archives),
Tom De Nies (iMinds - Ghent University),
Helena Deus (DERI Galway at the National University of Ireland, Galway, Ireland),
Simon Dobson (Invited expert),
Martin Doerr (Foundation for Research and Technology - Hellas(FORTH)),
Kai Eckert (Invited expert),
Jean-Pierre EVAIN (European Broadcasting Union, EBU-UER),
James Frew (Invited expert),
Irini Fundulaki (Foundation for Research and Technology - Hellas(FORTH)),
Daniel Garijo (Ontology Engineering Group, Universidad Politécnica de Madrid, Spain),
Yolanda Gil (Invited expert),
Ryan Golden (Oracle Corporation),
Paul Groth (Vrije Universiteit),
Olaf Hartig (Invited expert),
David Hau (National Cancer Institute, NCI),
Sandro Hawke (W3C/MIT),
Jörn Hees (German Research Center for Artificial Intelligence (DFKI) Gmbh),
Ivan Herman, (W3C/ERCIM),
Ralph Hodgson (TopQuadrant),
Hook Hua (Invited expert),
Trung Dong Huynh (University of Southampton),
Graham Klyne (University of Oxford),
Michael Lang (Revelytix, Inc.),
Timothy Lebo (Rensselaer Polytechnic Institute),
James McCusker (Rensselaer Polytechnic Institute),
Deborah McGuinness (Rensselaer Polytechnic Institute),
Simon Miles (Invited expert),
Paolo Missier (School of Computing Science, Newcastle university),
Luc Moreau (University of Southampton),
James Myers (Rensselaer Polytechnic Institute),
Vinh Nguyen (Wright State University),
Edoardo Pignotti (University of Aberdeen, Computing Science),
Paulo da Silva Pinheiro (Rensselaer Polytechnic Institute),
Carl Reed (Open Geospatial Consortium),
Adam Retter (Invited Expert),
Christine Runnegar (Invited expert),
Satya Sahoo (Invited expert),
David Schaengold (Revelytix, Inc.),
Daniel Schutzer (FSTC, Financial Services Technology Consortium),
Yogesh Simmhan (Invited expert),
Stian Soiland-Reyes (University of Manchester),
Eric Stephan (Pacific Northwest National Laboratory),
Linda Stewart (The National Archives),
Ed Summers (Library of Congress),
Maria Theodoridou (Foundation for Research and Technology - Hellas(FORTH)),
Ted Thibodeau (OpenLink Software Inc.),
Curt Tilmes (National Aeronautics and Space Administration),
Craig Trim (IBM Corporation),
Stephan Zednik (Rensselaer Polytechnic Institute),
Jun Zhao (University of Oxford),
Yuting Zhao (University of Aberdeen, Computing Science).

The editors also thank the developers of the tools that helped create the PROV-O ontology and portions of this document. Without these great tools, developing PROV-O would have been much less of a pleasure.

* Stanford's [Protege](http://protege.stanford.edu) for editing the ontology.
* Dave Beckett's [rapper](http://librdf.org/raptor/) for the many serialization checks of so many examples.
* Cosmin Basca's [SuRF](http://packages.python.org/SuRF/) for easing the construction of this page's cross reference section.
* The creators, contributors, and maintainers of [rdflib](https://github.com/RDFLib) for easing the construction of this page's cross reference section.
* Alvaro Graves' [LODSpeaKr](http://alangrafu.github.io/lodspeakr/) for constructing portions of this page with SPARQL queries of PROV-O.
* Silvio Peroni's [LODE](http://lode.sourceforge.net) for the CSS styling of this page's cross reference section.
* Robin Berjon's [respec](http://dev.w3.org/2009/dap/ReSpec.js/documentation.html) for handling the W3C styling.

G. References
-------------

### G.1 Normative references

[IRI]
:   M. Duerst, M. Suignard. [Internationalized Resource Identifiers (IRI).](http://www.ietf.org/rfc/rfc3987.txt) January 2005. Internet RFC 3987. URL: [http://www.ietf.org/rfc/rfc3987.txt](http://www.ietf.org/rfc/rfc3986.txt)

[OWL2-OVERVIEW]
:   W3C OWL Working Group. [OWL 2 Web Ontology Language: Overview.](https://www.w3.org/TR/2009/REC-owl2-overview-20091027/) 27 October 2009. W3C Recommendation. URL: [http://www.w3.org/TR/2009/REC-owl2-overview-20091027/](https://www.w3.org/TR/2009/REC-owl2-overview-20091027/)

[PROV-CONSTRAINTS]
:   James Cheney; Paolo Missier; Luc Moreau; eds. [Constraints of the PROV Data Model](https://www.w3.org/TR/2013/REC-prov-constraints-20130430/). 30 April 2013, W3C Recommendation. URL: [http://www.w3.org/TR/2013/REC-prov-constraints-20130430/](https://www.w3.org/TR/2013/REC-prov-constraints-20130430/)

[PROV-DM]
:   Luc Moreau; Paolo Missier; eds. [PROV-DM: The PROV Data Model](https://www.w3.org/TR/2013/REC-prov-dm-20130430/). 30 April 2013, W3C Recommendation. URL: [http://www.w3.org/TR/2013/REC-prov-dm-20130430/](https://www.w3.org/TR/2013/REC-prov-dm-20130430/)

[PROV-N]
:   Luc Moreau; Paolo Missier; eds. [PROV-N: The Provenance Notation](https://www.w3.org/TR/2013/REC-prov-n-20130430/). 30 April 2013, W3C Recommendation. URL: [http://www.w3.org/TR/2013/REC-prov-n-20130430/](https://www.w3.org/TR/2013/REC-prov-n-20130430/)

[RDF-CONCEPTS]
:   Graham Klyne; Jeremy J. Carroll. [Resource Description Framework (RDF): Concepts and Abstract Syntax.](https://www.w3.org/TR/2004/REC-rdf-concepts-20040210/) 10 February 2004. W3C Recommendation. URL: [http://www.w3.org/TR/2004/REC-rdf-concepts-20040210](https://www.w3.org/TR/2004/REC-rdf-concepts-20040210/)

[RFC2119]
:   S. Bradner. [Key words for use in RFCs to Indicate Requirement Levels.](http://www.ietf.org/rfc/rfc2119.txt) March 1997. Internet RFC 2119. URL: <http://www.ietf.org/rfc/rfc2119.txt>

[XMLSCHEMA11-2]
:   Henry S. Thompson; et al. [W3C XML Schema Definition Language (XSD) 1.1 Part 2: Datatypes.](https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/) 5 April 2012. W3C Recommendation URL: [http://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/](https://www.w3.org/TR/2012/REC-xmlschema11-2-20120405/)

### G.2 Informative references

[LD-Patterns-QR]
:   Leigh Dodds; Ian Davis. [Qualified Relation](http://patterns.dataincubator.org/book/qualified-relation.html). modified 31 May 2012, accessed 01 June 2012 URL: <http://patterns.dataincubator.org/book/qualified-relation.html>

[OWL2-PRIMER]
:   Pascal Hitzler; Markus Krötzsch; Bijan Parsia; Peter F. Patel-Schneider; Sebastian Rudolph. [OWL 2 Web Ontology Language:Primer.](https://www.w3.org/TR/2009/REC-owl2-primer-20091027/) 27 October 2009. W3C Recommendation. URL: [http://www.w3.org/TR/2009/REC-owl2-primer-20091027/](https://www.w3.org/TR/2009/REC-owl2-primer-20091027/)

[PROV-AQ]
:   Graham Klyne; Paul Groth; eds. [Provenance Access and Query](https://www.w3.org/TR/2013/NOTE-prov-aq-20130430/). 30 April 2013, W3C Note. URL: [http://www.w3.org/TR/2013/NOTE-prov-aq-20130430/](https://www.w3.org/TR/2013/NOTE-prov-aq-20130430/)

[PROV-DC]
:   Daniel Garijo; Kai Eckert; eds. [Dublin Core to PROV Mapping](https://www.w3.org/TR/2013/NOTE-prov-dc-20130430/). 30 April 2013, W3C Note. URL : [http://www.w3.org/TR/2013/NOTE-prov-dc-20130430/](https://www.w3.org/TR/2013/NOTE-prov-dc-20130430/)

[PROV-DICTIONARY]
:   Tom De Nies; Sam Coppens; eds. [PROV-Dictionary: Modeling Provenance for Dictionary Data Structures](https://www.w3.org/TR/2013/NOTE-prov-dictionary-20130430/). 30 April 2013, W3C Note. URL: [http://www.w3.org/TR/2013/NOTE-prov-dictionary-20130430/](https://www.w3.org/TR/2013/NOTE-prov-dictionary-20130430/)

[PROV-LINKS]
:   Luc Moreau; Timothy Lebo; eds. [Linking Across Provenance Bundles](https://www.w3.org/TR/2013/NOTE-prov-links-20130430/). 30 April 2013, W3C Note. URL: [http://www.w3.org/TR/2013/NOTE-prov-links-20130430/](https://www.w3.org/TR/2013/NOTE-prov-links-20130430/)

[PROV-OVERVIEW]
:   Paul Groth; Luc Moreau; eds. [PROV-OVERVIEW: An Overview of the PROV Family of Documents](https://www.w3.org/TR/2013/NOTE-prov-overview-20130430/). 30 April 2013, W3C Note. URL: [http://www.w3.org/TR/2013/NOTE-prov-overview-20130430/](https://www.w3.org/TR/2013/NOTE-prov-overview-20130430/)

[PROV-PRIMER]
:   Yolanda Gil; Simon Miles; eds. [PROV Model Primer](https://www.w3.org/TR/2013/NOTE-prov-primer-20130430/). 30 April 2013, W3C Note. URL: [http://www.w3.org/TR/2013/NOTE-prov-primer-20130430/](https://www.w3.org/TR/2013/NOTE-prov-primer-20130430/)

[PROV-SEM]
:   James Cheney; ed. [Semantics of the PROV Data Model](https://www.w3.org/TR/2013/NOTE-prov-sem-20130430). 30 April 2013, W3C Note. URL: [http://www.w3.org/TR/2013/NOTE-prov-sem-20130430](https://www.w3.org/TR/2013/NOTE-prov-sem-20130430).

[PROV-XML]
:   Hook Hua; Curt Tilmes; Stephan Zednik; eds. [PROV-XML: The PROV XML Schema](https://www.w3.org/TR/2013/NOTE-prov-xml-20130430/). 30 April 2013, W3C Note. URL: [http://www.w3.org/TR/2013/NOTE-prov-xml-20130430/](https://www.w3.org/TR/2013/NOTE-prov-xml-20130430/)

[RDF-SCHEMA]
:   Dan Brickley; Ramanathan V. Guha. [RDF Vocabulary Description Language 1.0: RDF Schema.](https://www.w3.org/TR/2004/REC-rdf-schema-20040210/) 10 February 2004. W3C Recommendation. URL: [http://www.w3.org/TR/2004/REC-rdf-schema-20040210](https://www.w3.org/TR/2004/REC-rdf-schema-20040210/)

[TRIG]
:   Chris Bizer; Richard Cyganiak. [The TriG Syntax](http://wifo5-03.informatik.uni-mannheim.de/bizer/trig/). modified 30 July 2007, accessed 05 November 2012 URL: <http://wifo5-03.informatik.uni-mannheim.de/bizer/trig/>