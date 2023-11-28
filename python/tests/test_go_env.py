from go_gym import GoEnv


def test_go_env():
    print("Testing GoEnv")
    env = GoEnv(board_size=9)
    print('Successfully created GoEnv')

    # try and step
    obs = env.reset()
    print('Successfully reset GoEnv')
    
    # take an action
    env.step(40)
    print('Successfully took action')

    # render
    env.render()
    print('Successfully rendered')

    # try putting an invalid move
    #env.step(-1)


if __name__ == '__main__':
    test_go_env()

