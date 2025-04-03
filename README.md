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
| Component          | Checkpoint 1 (MM/DD)       | Checkpoint 2 (MM/DD)       |
|--------------------|----------------------------|----------------------------|
| Game Logic         | Basic Q&A flow             | Multiplayer support        |
| Encryption         | Plaintext scoring          | Full AES implementation    |
| UI                 | CLI interface              | WebSocket API              |

## Possible Challenges
1. **Cross-platform Crypto**  
   Ensuring consistent encryption across Windows/macOS/Linux  
2. **Async Redemptions**  
   Handling concurrent point claims safely  
3. **Question Variety**  
   Dynamic loading of new question packs  

