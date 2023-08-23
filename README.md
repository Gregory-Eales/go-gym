# go-gym
open ai style go environment written in rust


.
├── src                             # Rust source files
│   ├── lib.rs                      # The main library file which will define your Python module
│   ├── game.rs                     # Game rules and logic
│   ├── state.rs                    # Game state management
│   ├── actions.rs                  # Legal move generation
│   ├── renderer.rs                 # Rendering the game board
│   └── tests                       # Folder for Rust test modules
│       ├── game_tests.rs
│       ├── state_tests.rs
│       ├── actions_tests.rs
│       └── renderer_tests.rs
├── target                          # The build output directory, auto-generated by Cargo
├── examples                        # Optional directory for example uses of your library
├── Cargo.toml                      # Manifest file that contains metadata and dependencies
├── Cargo.lock                      # Autogenerated by Cargo, contains exact versions of dependencies
├── setup.py                        # Python setup file for packaging your library
├── README.md                       # The top-level description of your library
├── Python                          # Python-related files
│   ├── go_env                      # Python version of the Go environment
│   │   ├── __init__.py             
│   │   └── go_env.py
│   ├── tests                       # Python tests folder
│   │   └── test_go_env.py
│   └── examples                    # Example Python scripts using the Go environment
│       └── basic_usage.py
└── .gitignore                      # Specifies files that Git should ignore

