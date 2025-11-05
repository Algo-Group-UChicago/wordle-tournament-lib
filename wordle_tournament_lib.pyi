"""
Wordle Tournament Library

Library written in Rust for running aWordle tournament. Written by Kathir Meyyappan and Akash Piya."""

from typing import List, Tuple

class WordleHint:
    """
    Represents a Wordle guess with its corresponding hint pattern. Lists of these are passed to the guess function 
    in the tournament.
    
    Hint characters:
        - 'O': Correct letter in correct position (🟩 green)
        - '~': Letter is present but in wrong position (🟨 yellow)  
        - 'X': Letter is absent from the word (⬜ gray)
    
    Example:
        >>> hint = WordleHint("hello", "O~XX~")
        >>> hint.visualize_hint()
        H E L L O
        🟩 🟨 ⬜ ⬜ 🟨
    """
    
    word: str
    """The 5-letter word that was guessed."""
    
    def __init__(self, word: str, hints: str) -> None:
        """
        Create a new WordleHint.
        
        Args:
            word: A 5-letter word
            hints: A 5-character string of hint characters ('O', '~', or 'X')
            
        Raises:
            ValueError: If word is not 5 letters long
            ValueError: If hints length doesn't match word length
            ValueError: If any hint character is invalid
        """
        ...
    
    @property
    def hints(self) -> List[str]:
        """
        Get the hint pattern as a list of strings.
        
        Returns:
            List of hint characters: ['O', '~', 'X', ...]
        """
        ...
    
    @property
    def word_hint_pairs(self) -> List[Tuple[str, str]]:
        """
        Get pairs of (letter, hint) for each position.
        
        Returns:
            List of tuples: [('h', 'O'), ('e', '~'), ...]
        """
        ...
    
    def visualize_hint(self) -> None:
        """
        Print a visual representation of the hint using emoji squares.
        
        Prints two lines:
        - Line 1: Letters in uppercase separated by spaces
        - Line 2: Colored squares (🟩/🟨/⬜) matching the hint pattern
        
        Example output:
            H E L L O
            🟩 🟨 ⬜ ⬜ 🟨
        """
        ...
    
    def __repr__(self) -> str:
        """Return string representation of WordleHint."""
        ...


class UChicagoWordleBotBase:
    """
    Base class for UChicago Wordle Tournament bots.

    This class provides the infrastructure for running wordle bots in the tournament.
    Subclass this and implement the `guess()` method to create your bot.

    Example:
        >>> class MyBot(UChicagoWordleBotBase):
        ...     def __init__(self, team_id: str):
        ...         pass
        ...
        ...     def guess(self, hints: List[WordleHint]) -> str:
        ...         # Your guessing logic here
        ...         return "crane"
        >>>
        >>> bot = MyBot("team-uuid")
    """

    team_id: str
    """Unique identifier for the team."""

    def __init__(self, team_id: str) -> None:
        """
        Initialize a new Wordle bot.

        Args:
            team_id: Unique identifier for your team
        """
        ...

    def guess(self, hints: List[WordleHint]) -> str:
        """
        Make a guess based on previous hints.

        This is an abstract method that must be overridden by subclasses.

        Args:
            hints: List of previous guesses and their corresponding hints.
                   Empty list on first guess.

        Returns:
            A 5-letter word guess

        Raises:
            NotImplementedError: If not overridden in subclass

        Example:
            >>> def guess(self, hints: List[WordleHint]) -> str:
            ...     if not hints:
            ...         return "crane"  # First guess
            ...     # Analyze hints and return next guess
            ...     return "stare"
        """
        ...

