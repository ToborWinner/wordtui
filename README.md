# Wordtui - The TUI to learn new words in a foreign language
Wordtui is a terminal user interface (TUI) to learn new words in a foreign language. It's a basic TUI that allows you to study a list of words approximately in order, while still often randomly asking for previously learned words to make sure you don't forget them. It's written in Rust.

## Word-picking algorithm
The next word in Wordtui is picked based on the following algorithm. We'll call it current word.
First, the first word where the streak (correctly gussed in a row) is lower than 5 is found.
There is a 1/5th chance that the word is picked from all words that come before the current word.
Otherwise, the word is randomly picked between the next 9 words from the current word and the current word itself.

This ensures that if you randomly get picked to revise a previous word (the 1/5th chance) and mess up, you'll have to guess that word 5 times in a row along with the words in that area before being allowed to go back to what you were studying. Forgetting words is not an option!

## Setup
To setup a language, you should create a `language.json` file in the wordtui config directory (`~/.config/wordtui/` on Linux). The file should have the following format:
```json
{
    "name": "Language name",
    "words": [
        {
            "word": "word1",
            "url": "url to a dictionary definition. Example: /wiki/hello#English",
            "answer": "word1 translation. This is optional. When not provided, the first time you see the word, you'll be asked to provide the translation after seeing the definition (included below in the extract).",
            "extract": [
                {
                    "name": "Article",
                    "content": "Section content. This is the definition that will be shown to the user when they mess up or when first defining the translation. For subsections, they can be wrapped ==== like this ====. Those will be shown in red. Empty lines will not be shown."
                },
                {
                    "name": "Pronoun",
                    "content": "section content"
                },
                {
                    "name": "Noun",
                    "content": "section content"
                },
                {
                    "name": "Pronunciation",
                    "content": "section content"
                }
            ]
        }
    ]
}
```

After using Wordtui, the `language.json` file will be updated with the streaks and stats of each word. To reset the streaks, you can delete the `language.json` file and add a fresh one.

## Usage
To use Wordtui, you can just run the command in a CLI and the TUI will open.

To quit the TUI, you can press `q`.
To scroll up and down, you can use `j` and `k`, as well as `d` and `u` (although they will move you more lines at once).
To go from the definition to the next word, you can press `n`.
On the definition page, to change the translation, you can press `r` and type the new answer. Press `Enter` to save it.
On the question page, you can use `Enter` to submit the answer. You can use `Esc` to unselect the textbox. It can be selected again with `i`. To change the translation when on the question page, you can press `r` and type the new answer. Press `Enter` to save it.
When writing, `Tab` can be used to toggle the next character as a special character. This is for example useful for converting `a` to `ä`.
In the definition page, after messing up a question, you can press `f` to mark it as correct instead (in case you were right, for example a misspelling).
In the question page, `s` will increase the streak by one. This is useful when you know the word (and are sure you know the words around) and want to reach streak of 5 quicker.
