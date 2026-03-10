FROM archlinux:multilib-devel

RUN pacman -Syyu --noconfirm
RUN pacman -S lightdm git --noconfirm --needed
