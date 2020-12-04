# v0.2

## Arrangements
This release adds `Arrangement`s, which allow you to stitch together
multiple sounds into a larger sound that you can play instances of.

The main use case for this is setting up seamless loops, including
looping songs with intros. The previous release could create seamless
loops, but only if the sound didn't have an intro.

Functions that dealt with instances of sounds now deal with instances
of sounds *or* arrangements. `InstanceSettings` has become
`PlayableSettings` to correspond to the new `Playable` enum.

Arrangements also help reduce some complexity from the code for
instances and make some more unusual behaviors, like playing
instances backwards or rewinding instances (not implemented yet,
but planned), easier to reason about.

I'm sure there's other good uses for arrangements, too! But I think
the most common use case will be songs with intros.

## Other changes
- Remove `SoundMetadata` and move `semantic_duration` into `PlayableSettings`
- Rename `StereoSample` to `Frame`
- Added support for panning instances:
	- Added `Frame::panned`
	- Added a `panning` field and method to `InstanceSettings`
	- Added `AudioManager::set_instance_panning`
	- Added `Sequence::set_instance_panning`
- Added parameter mappings, which allow `Value`s to map to parameters with
custom scaling and offsets. `Value::Parameter` now contains a `Mapping`
as its second piece of data. `ParameterId`s can be converted into
`Value::Parameter`s with the default 1:1 mapping.
- Changed `Tween` to a C-style struct with named fields:
	- `duration` - the duration of the tween
	- `easing` - the easing function to use (linear, power, etc.)
	- `ease_direction` - the easing direction (in, out, in-out)
- Added chainable methods to more settings structs
- Replaced `AudioManager::events` with `AudioManager::pop_event`
- Add `Sequence:pause/stop/resume_sequence`

# v0.1
First public release