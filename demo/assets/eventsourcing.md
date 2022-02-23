# Event Sourcing

---

## Capture state change as a sequence of events

---

## How we got where we are

---

## Adjust state to handle retroactive changes

---

## Query log to get past state

---

## All change initiated by event objects

---

## Rebuild state from event log

---

## Replay events to debug with real data

---

## Temporal query to find state at any point in time

---

## Official state is either state or the event log

---

## State stored in memory

## with snapshots stored to disk

---

## State stored in a database with event log

## used for special processing and audits

---

## Event reversing

Change as the difference of state

or

Store data needed for reversal

---

## Reprocess events to take advantag

## e of new features or bug fixes

---

## Replay events to test upgrades

---

## Handle external updates with a gateway which

## is aware that the system is processing replays

---

## External queries store responses in gateway,

## or queries for date specific value

---

## Loosely coupled parallel systems

## excellent for horizontal scaling

---

## Separate reading from writing

---

## New applications can easily be added

## by tapping in to the event stream

---

# Olle Wreede

@ollej
