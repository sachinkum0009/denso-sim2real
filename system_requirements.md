# System Requirements

The main objective of `PolicyBridge` is to provide a library with can be used to load the trained RL Policy and then use it execute on the real robot.

## Functional Req

- The system loads policy and run inference engine to output the actions.
- The system must ingest raw hardware sensors (based on the RL policy).

## Non Functional Req

- The execution loop must run deterministically at a fixed frequency (e.g., 50 Hz).
- The lib should be portable and used by standard ROS2 frameworks.

## Domain Req

- Provide configurable scaling factor or filters (like low pass or kalman filters) to smooth out the transition to real, noisy sensor streams.
