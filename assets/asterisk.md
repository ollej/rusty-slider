## Webbaserad röstmenystyrning
## för telefonväxelsystem

Olle Wreede

2007

---

## Översikt

* Bakgrund
* Problembeskrivning
* PBX
* Asterisk
* Asterisks arkitektur

---

## Översikt fortsättning

* Dialplan
* AGI
* AGI-system
* Manager API
* Implementering

---

## Bakgrund

* Långa telefonköer
* Dålig kontroll för kunden
* Röstmenyer är svåra att använda
* Statisk info på supportwebbplatser
* Sammankoppling av webb och samtal
* Större kontroll
* Snabbare service

---

## Problembeskrivning

* Gamla växlar är svårkonfigurerade
* Mjukvarubaserad växel – Asterisk
* Vilka möjligheter finns i Asterisk?
* Koppla samman webb med telefon

---

## Asterisk

* Open Source
* Lättare att bygga ut
* Stödjer standarder

---

## PBX och telefonväxlar

Private Branch Exchange

---

## Vanliga funktioner i PBX

* Fler anslutningar än telefonlinjer
* Interna samtal
* Röstbrevlådor
* Vidarekoppling
* Väntmusik
* Vissa hanterar vanlig telefoni och VoIP

---

## Mjukvarubaserad växel

<!-- bild saknas -->

* PC med instickskort och Asterisk
* Telefonnätet kopplat till PC:n
* Telefoner kopplade till PC:n
* IP-telefon inkopplade via ethernet

---

## Asterisk

* Mjukvarubaserad
* Öppen källkod – GPL
* Utvecklas och supportas av Digium
* Klarar vanliga telenätet och VoIP
* Stödjer många olika hårdvaror
* Konfigurerbar och programmerbar
* Utvecklas aktivt

---

## Asterisks arkitektur

Kärna för intern funktionalitet

---

## Fyra API:er

**Channel API**

Hanterar olika typer av telefonkanaler

**Application API**

Hanterar tjänster som röstbrevlåda och konferenser

**Codec Translator API**

Laddar codecs för olika ljudformat

**File Format API**

Läser/Skriver data

---

## Asterisks Dialplan

* Innehåller instruktioner för samtal
* Detaljstyr samtalens väg genom systemet
* Listar anknytningar
* Läser knapptryckningar
* Spelar upp ljud
* Startar applikationer
* Kraftfullt och avancerat

---

## Asterisk Gateway Interface

* Gränssnitt för kommunikation

mellan Asterisks dialplan och applikation

* Läser/skriver via FIFO
* Applikationerna startas från dialplan
* AGI-kommandon

för läsning av knapptryckningar, uppspelning av ljud osv

* Kan skrivas i vilket språk som helst

---

## AGI-system

* Klassbibliotek för AGI-program
* Förenklar kommunikationen
* Direkt stöd för AGI-kommandon

---

## AGI finns till många språk

* phpAGI
* Asterisk PHP
* Asterisk-java
* Asterisk Perl Library

---

## Manager API

* Används av externa klienter
* Visar information om samtal
* Omstyrning av pågående samtal
* Paket skickas via TCP/IP-ström
* Data skickas asynkront
* Stödjer ett flertal olika actions

---

## Implementering

* Två delar – AGI-applikation och webbsystem
* Systemen måste kopplas samman
* Kan göras med kod som knappas in på telefonen
* AGI-applikationen tar över samtalet från dialplan

---

## Webbsystem

* Funktionalitet motsvarande röstmeny
* Tillåter tydligare information
* Kan kopplas samman med webbsupport
* Visar kod för sammankoppling
* Kan visa status om pågående samtal
* Ifyllning av information under väntan

---

## AGI-applikation

* Startas från dialplan
* Läser in webbkod
* Läser data från webb-sessionen
* Synkar samtalet via databas
* Utnyttjar webbsystemets menysystem
* Agerar slav till webbsystemet

---

## Slutledning

* Fullt möjligt att utveckla
* Kan kodas i olika språk
* Komplicerat
* Kräver många olika delar av Asterisks API:er
* Kräver inga dyra system
* Kan byggas ut med fler funktioner

---

# Olle Wreede
