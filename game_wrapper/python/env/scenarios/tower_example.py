from scaii.env.sky_rts.env import SkyRtsEnv, MoveList


class TowerAction(MoveList):
    def __init__(self, discrete_actions=None, continuous_actions=None, env_actions=None):
        super().__init__(discrete_actions=discrete_actions,
                         continuous_actions=continuous_actions, env_actions=env_actions)

    def attack_quadrant(self, quadrant):
        super().move_unit(
            self.state.id_list[0], "attack", self.state.id_list[quadrant])


class TowerExample(SkyRtsEnv):
    def __init__(self):
        super().__init__(action_type=MoveList)

        super().load_scenario("tower_example")

    def new_action(self):
        act = super().new_action(self)
        act.state = self.state
