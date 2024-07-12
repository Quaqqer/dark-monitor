# Sway wallpaper example
dark-monitor monitor --default-as light \
  --on-dark 'swaymsg "output * bg ~/Pictures/wallpaper_dark.png"' \
  --on-light 'swaymsg "output * bg ~/Pictures/wallpaper_light.png"'
