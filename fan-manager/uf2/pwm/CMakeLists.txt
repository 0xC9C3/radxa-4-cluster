add_executable(
pwm_fan
pwm_fan.c
)

# pull in common dependencies and additional pwm hardware support
target_link_libraries(pwm_fan pico_stdlib hardware_pwm)

# create map/bin/hex file etc.
pico_add_extra_outputs(pwm_fan)
