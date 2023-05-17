#!/bin/sh
printf '%b' '\e]P0{{ colors.color0 | strip }}
			 \e]P1{{ colors.color1 | strip }}
			 \e]P2{{ colors.color2 | strip }}
			 \e]P3{{ colors.color3 | strip }}
			 \e]P4{{ colors.color4 | strip }}
			 \e]P5{{ colors.color5 | strip }}
			 \e]P6{{ colors.color6 | strip }}
			 \e]P7{{ colors.color7 | strip }}
			 \e]P8{{ colors.color8 | strip }}
			 \e]P9{{ colors.color9 | strip }}
			 \e]PA{{ colors.color10 | strip }}
			 \e]PB{{ colors.color11 | strip }}
			 \e]PC{{ colors.color12 | strip }}
			 \e]PD{{ colors.color13 | strip }}
			 \e]PE{{ colors.color14 | strip }}
			 \e]PF{{ colors.color15 | strip }}
			 \ec'
