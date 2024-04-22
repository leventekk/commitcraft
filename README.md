# CommitCraft

---

**Description:**
CommitCraft is a command-line interface (CLI) application written in Rust, designed to facilitate the generation of Git commits from stashed changes. With CommitCraft, you can efficiently create Git commits in two distinct formats: raw commit and conventional commit. Whether you prefer a simple, raw format or adhere to the conventional commit standard, CommitCraft has got you covered.

**Features:**
- **Stash-based Commit Generation:** CommitCraft enables you to generate Git commits directly from stashed changes.
- **Two Commit Formats:** Choose between two commit formats: raw commit or conventional commit, based on your preferences and project requirements.
- **Flexible Configuration:** Easily configure CommitCraft with various options, including API key management for integration with external services.

**Usage:**

```bash
commitcraft [--format <format>]
```

**Flags:**  
- `--format <format>`: Specifies the desired commit format. Accepts values: `raw` or `conventional`. Default is `raw`.

**Commands:**  
- `config`: Configure CommitCraft settings.
  - `--api-key <api_key>`: Set the OpenAI API key for enhanced functionality.

**Examples:**  
1. Generate a raw format commit:  

```bash
commitcraft --format raw
```

2. Generate a conventional commit:  

```bash
commitcraft --format conventional
```
3. Configure CommitCraft with an API key:  

```bash
commitcraft config --api-key <your_api_key>
```

