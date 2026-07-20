#### 1. Get API Key from Google
- Go to [Google AI Studio](https://aistudio.google.com/).
- Go to `Dashboard` and press button `Create API key`.

#### 2. Export API Key
```shell
export GEMINI_API_KEY="YOUR_API_KEY"
```

#### 3. Install `mdfried` for Viewing Markdown Files

```shell
$ pacman -S mdfried
```

#### 4. Run Binary
```shell
$ cargo run -- `filename` > summary.md
```

#### 5. View Summary
```shell
$ mdfried summary.md
$ mdfried summary.md -w # watch mode
```
