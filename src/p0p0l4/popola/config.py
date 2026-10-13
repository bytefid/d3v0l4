from dataclasses import dataclass


@dataclass(slots=True)
class PopolaConfig:
    sense_block_size: int = 2

    @classmethod
    def new(cls):
        return cls()

    def get_sense_block_size(self):
        return self.sense_block_size
