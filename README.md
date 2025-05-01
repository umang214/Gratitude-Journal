# Gratitude Journal

## Table of Contents
- [Project Title](#project-title)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Contract Details](#contract-details)

## Project Title

**Gratitude Journal**

## Project Description

A decentralized, on-chain journal for logging things you're grateful for. Users can privately and immutably store gratitude entries, promoting mindfulness and self-awareness through blockchain technology.

## Project Vision

To create a simple, meaningful tool that encourages users to focus on the positive moments of life while offering privacy and ownership of their entries through decentralization.

## Key Features

- ✍️ **Log Entries**: Users can add gratitude notes tied to their address.
- 📜 **View Past Entries**: Retrieve previously logged gratitude items anytime.
- 📊 **Entry Counter**: Track how many entries you've logged over time.
- 🔐 **User-Only Access**: Only authenticated users can add entries for themselves.

## Contract Details

### Contract Address: CAWM6VSA5W2JUQOKVG2YGMUP2JLDNOIP455ORQR3BJVST4X6YYE5EP23

This Soroban contract supports:

### 1. `add_entry(user: Address, entry: String)`
Stores a gratitude entry for the authenticated user.

### 2. `get_entries(user: Address) -> Vec<String>`
Returns all entries made by a specific user.

### 3. `count_entries(user: Address) -> u32`
Returns the total number of gratitude entries stored by a user.

---

**Start your gratitude journey, one entry at a time.** 🙏💛  
Built with Soroban for soul & self-care.
