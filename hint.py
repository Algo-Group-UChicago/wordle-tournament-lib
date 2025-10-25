from enum import Enum

class HINT_TYPE(Enum):
    CORRECT = 'W'
    PRESENT = '*'
    ABSENT = 'L'

class WordleHint:
    def __init__(self, word: str, hints: list[HINT_TYPE]):
        assert len(word) == 5, "Word must be 5 letters long"
        assert len(word) == len(hints), "Word and hints must have the same length"
        
        self.results = [(word[i], hints[i]) for i in range(5)]

    def visualize_hint(self):
        letters = []
        squares = []
        for letter, hint in self.results:
            letters.append(letter.upper())
            if hint == HINT_TYPE.CORRECT:
                squares.append('🟩')
            elif hint == HINT_TYPE.PRESENT:
                squares.append('🟨')
            else:  # ABSENT
                squares.append('⬜')
        print(' '.join(letters))
        print(' '.join(squares))

    def __repr__(self):
        return f"WordleHint(word={self.word}, hints={self.hints})"