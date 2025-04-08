# Project Name: Secure Trivia Game

## Group Information
- **Group Name**: Cryptic Wizards 
- **Members**:
  - Hakeem Muritala (NETID:hmuri)
  - Angela Li (NETID:meixian2)

## Project Introduction
**Description**:  
A Rust-based trivia game with encrypted point storage, where players earn points by answering questions and can later redeem them to use towards hints for the trivia game.  

**Why Rust?**  
- Memory safety for secure encryption  
- Performance for real-time gameplay  
- Strong typing prevents score manipulation bugs  

## Technical Overview
### Core Components
1. **Question Engine**  
   - JSON-based question bank  
   - Category filtering  
2. **Encryption Module**  
   - AES-256 via `aes-gcm` crate  
   - Secure key management  
3. **Point System**  
   - Sled embedded database  
   - Atomic transactions  



### Roadmap 

| Component       | Check Point 1                | Check Point 2                |
|-----------------|-------------------------------|-------------------------------|
| **Game Core**   | - basic questions<br>- Point tracking<br>- Terminal input/output | - Hint purchasing system<br>- Difficulty scaling<br>- Score persistence |
| **Encryption**| - Mock encryption stub<br>- Basic point storage          |  - AES-256 implementation<br>- Secure key management<br>         |
| **Interface**   | - Simple text menus           | - Colored question display    |


## Possible Challenges
1. **Cross-platform Crypto**  
   Ensuring consistent encryption across Windows/macOS/Linux  
2. **Async Redemptions**  
   Handling concurrent point claims safely  
3. **Question Variety**  
   Dynamic loading of new question packs  
   Question variety for certain categories

## references 
1. **Trivia Data Set**  
   https://github.com/uberspot/OpenTriviaQA/tree/master
   
