from scaii.env.state import State

# pylint: disable=locally-disabled, E1101


class SkyState(State):
    def __init__(self, typed_reward=None, reward=None, terminal=False, state=None, env_state=None):
        import numpy as np

        super().__init__(typed_reward, reward, terminal, state, env_state)

        id_map = state[:, :, 0].astype(np.int)
        self.id_list = np.unique(id_map).astype(np.int)
        self.id_map = id_map

        self.state = np.empty(
            (state.shape[0], state.shape[1], 6), dtype=np.float)

        # Normalize HP
        self.state[:, :, 0] = state[:, :, 1] / 500.0

        unit_ids = state[:, :, 2].astype(np.int)

        self.state[:, :, 1:4] = np.equal.outer(unit_ids, np.unique(unit_ids)).astype(np.float)[
            :, :, 1:]

        faction_ids = state[:, :, 2].astype(np.int)
        self.state[:, :, 4:] = np.equal.outer(
            faction_ids, np.unique(faction_ids)).astype(np.float)[:, :, 1:]

        self.old_state = state
