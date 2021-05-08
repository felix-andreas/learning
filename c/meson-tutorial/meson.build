project(
  'tutorial',
  'c',
  version : '0.1',
  default_options : ['warning_level=3']
)

gtkdep = dependency('gtk+-3.0')

exe = executable(
  'tutorial',
  'main.c',
  dependencies: gtkdep
)

test('basic', exe)
