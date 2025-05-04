# Project Name: Secure Trivia Game

## Group Information
- **Group Name**: Cryptic Wizards 
- **Members**:
  - Hakeem Muritala (NETID:hmuri)
  - Angela Li (NETID:meixian2)

## Project Introduction
**Description**:  
A terminal-based trivia game built in Rust, where players earn encrypted points by answering questions correctly. These points can later be used to unlock hints. The game ensures point integrity using a custom XOR-based encryption method.

**Why Rust?**  
- Memory safety for secure encryption  
- Performance for real-time gameplay  
- Strong typing prevents score manipulation bugs
- Lightweight and secure XOR-based logic for obfuscating player score

## Technical Overview
### Core Components
1. **Question Engine**  
   - JSON-based question bank stored in `data/questions.json`
   - Category filtering and random question selection via `game_core.rs`
2. **Encryption Module**  
   - XOR-based encryption
   - Implemented in `encryption.rs`
   - Symmetric encryption where `encrypted = value ^ key`, and decryption re-applies the XOR
3. **Point System**  
   - Encrypted point tracking using the `SecurePointSystem` struct in `point_system.rs`
   - Supports earning points and spending them for hints, with both operations using encryption/decryption

4. **Terminal Game Interface**
   - Fully playable in terminal with interactive input/output
   - Players select category, answer questions, and manage hint usage using terminal prompts



### Roadmap 

| Component       | Checkpoint 1                                      | Checkpoint 2                                               |
|----------------|---------------------------------------------------|-------------------------------------------------------------|
| **Game Core**  | Basic question loading                            | Hint purchasing logic<br>Answer validation<br>Score updating |
|                | Category filtering                                |                                                             |
|                | Terminal I/O interaction                          |                                                             |
| **Encryption** | XOR Encryptor struct                              | Decryption-integrated point system                          |
|                | Initial point encoding                            | Bidirectional XOR logic                                     |
| **Interface**  | Basic terminal prompt                             | Interactive loop<br>Hint request and validation handling     |
|                | User category input                               |                                                             |

## Possible Challenges
1. **Cross-platform Crypto**  
   Ensuring consistent encryption across Windows/macOS/Linux  
2. **Terminal Input Robustness**  
   Handled invalid inputs, case insensitivity, and looped interactions clearly in CLI
3. **Hint Mechanism Logic**  
   Integrated conditional point checking and hint display with real-time score updates
4. **Designing Secure XOR Logic**  
   Needed to ensure reversible encryption while keeping the implementation lightweight and effective

## references 
1. **Trivia Data Set**  
   https://github.com/uberspot/OpenTriviaQA/tree/master
   
2. **Rust Crates Used**   
   `serde`, `serde_json`: for loading JSON questions
   `rand`: for random question selection
   
