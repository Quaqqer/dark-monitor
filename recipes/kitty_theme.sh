# Add 'include theme.conf' to kitty.conf
dark-monitor monitor --default-as light \
  --on-dark 'ln  -fs ~/.config/kitty/theme_dark.conf  ~/.config/kitty/theme.conf && killall -s USR1 kitty' \
  --on-light 'ln -fs ~/.config/kitty/theme_light.conf ~/.config/kitty/theme.conf && killall -s USR1 kitty'
