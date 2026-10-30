#!/usr/bin/env bash
# Build predatorsense and install it with the udev rule that lets it run
# without root, plus a service that restores your lighting at boot and resume.
#
#   ./install.sh            build, install the binary, udev rule and service
#   ./install.sh uninstall  remove all three
#
# Run as your normal user; it asks for sudo only for the install steps, so
# cargo never runs as root and target/ stays yours.

set -euo pipefail

BIN_SRC="target/release/predatorsense"
BIN_DST="/usr/local/bin/predatorsense"
RULE_SRC="packaging/udev/70-predatorsense.rules"
RULE_DST="/etc/udev/rules.d/70-predatorsense.rules"
UNIT_SRC="packaging/systemd/predatorsense-apply@.service"
UNIT_DST="/etc/systemd/system/predatorsense-apply@.service"

cd "$(dirname "$0")"

if [[ $EUID -eq 0 ]]; then
    echo "run this as your normal user, not root (it uses sudo where needed)" >&2
    exit 1
fi

# One instance per user; this script sets it up for whoever runs it.
UNIT_INSTANCE="predatorsense-apply@$USER.service"

reload_udev() {
    sudo udevadm control --reload
    # Re-run the rules on the existing nodes so permissions change now,
    # not on the next boot.
    sudo udevadm trigger --subsystem-match=hidraw
    sudo udevadm settle
}

# Not named `install`: that would shadow the coreutils install(1) used below.
do_install() {
    echo "==> building"
    cargo build --release

    echo "==> installing $BIN_DST"
    sudo install -Dm755 "$BIN_SRC" "$BIN_DST"

    echo "==> installing $RULE_DST"
    sudo install -Dm644 "$RULE_SRC" "$RULE_DST"
    reload_udev

    echo "==> device permissions"
    local found=0
    for node in /sys/class/hidraw/hidraw*; do
        local id
        id=$(basename "$(readlink -f "$node/device")")
        case "$id" in
            0018:0CF2:5130.* | 0018:1025:174B.*)
                ls -l "/dev/$(basename "$node")"
                found=1
                ;;
        esac
    done
    if [[ $found -eq 0 ]]; then
        echo "warning: no matching controller found; is this an Acer Predator with these devices?" >&2
    fi

    if ! getent group plugdev >/dev/null; then
        echo "note: no plugdev group on this system; only the logged-in user (uaccess) gets access" >&2
    elif ! id -nG "$USER" | tr ' ' '\n' | grep -qx plugdev; then
        echo "note: $USER is not in plugdev; the TUI works while logged in, but a boot service running as $USER will not."
        echo "      to fix: sudo usermod -aG plugdev $USER   (then log out and back in)"
    fi

    echo "==> installing $UNIT_DST"
    sudo install -Dm644 "$UNIT_SRC" "$UNIT_DST"
    sudo systemctl daemon-reload

    echo "==> enabling $UNIT_INSTANCE (boot and resume)"
    sudo systemctl enable "$UNIT_INSTANCE"
    # Run it once now. Not fatal: with no profile saved yet, apply fails
    # until you set the lighting in the TUI.
    if ! sudo systemctl start "$UNIT_INSTANCE"; then
        echo "warning: $UNIT_INSTANCE failed; see 'systemctl status $UNIT_INSTANCE'" >&2
        echo "         and ~/.local/state/predatorsense/predatorsense.log" >&2
    fi

    echo "==> done; run 'predatorsense' without sudo"
}

do_uninstall() {
    echo "==> disabling $UNIT_INSTANCE"
    # Tolerate a unit that was never enabled, so uninstall can be rerun.
    sudo systemctl disable "$UNIT_INSTANCE" 2>/dev/null || true

    echo "==> removing $BIN_DST, $RULE_DST and $UNIT_DST"
    sudo rm -f "$BIN_DST" "$RULE_DST" "$UNIT_DST"
    sudo systemctl daemon-reload
    reload_udev
    echo "==> done"
}

case "${1:-install}" in
    install) do_install ;;
    uninstall) do_uninstall ;;
    *)
        echo "usage: $0 [install|uninstall]" >&2
        exit 1
        ;;
esac
