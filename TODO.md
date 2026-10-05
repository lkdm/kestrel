# TODO

## Split into crates

Split all adapters into crates:
- cli
- command

## Check ergonomics

Rules;
- A user should be able to read the PASS list alone and understand what Kestrel has verified about their machine.
- Then only expand when there's something the user needs to understand or act on

- Optional remeditation should be part of the check
  - It should be an Vec or enum of; Guide, Command, Function, Recommendation
  - Make `Command` command pattern
  - `--fix` `--dry-run` (note this should be a feature)

- Rename ERRORS to UNKNOWN

```

┌──────────────────────────────────────────
│ Check
│
│   ID
│   Title
│   Requirement
│   Observed state
│   Status
│   Evidence
│   Severity
│
│ Remediation
│   Summary
│   Guide URL
│   Optional safe action
│   Required privileges
└──────────────────────────────────────────

                    Normal       Detailed
Passed              ✓ title      ✓ title + explanation
Failed              ! title      ! title + explanation + evidence
Unknown             ? title      ? title + reason + evidence
Remediation         on failure   on failure
Guide               optional     visible
Fix                 optional     visible when available
```

```
PASSED
  ✓ System settings require authentication
  ✓ Password is required immediately after inactivity
  ✓ Automatic login is disabled
  ✓ Firewall is enabled
  ✓ System Integrity Protection is enabled
  ✓ macOS is up to date
  ✓ Remote Login is disabled
  ✓ Gatekeeper is enabled
  ✓ FileVault disk encryption is enabled
  ✓ Firewall blocks incoming connections
  ✓ Wi-Fi connection is encrypted
  ✓ Printer Sharing is disabled
  ✓ AirDrop is restricted
  ✓ SMB File Sharing is disabled
  ✓ Media Sharing is disabled
  ✓ Internet Sharing is disabled
  ✓ AirPlay Receiver is disabled
  ✓ SSH private keys meet cryptographic requirements
  ✓ Docker is running rootlessly
  ✓ Homebrew packages are up to date
PASSED

  ✓ Firewall is enabled
    └─ Requirement met: incoming connection filtering is active

  ✓ FileVault is enabled
    └─ Requirement met: startup disk is encrypted

  ✓ AirDrop is restricted
    └─ Requirement met: receiving is limited to Contacts Only or disabled

NEEDS ATTENTION

  ! Password is not required immediately after inactivity
    └─ Enable "Require password immediately" after the screen saver starts

  ! Daily account has administrator privileges
    └─ Use a Standard account for routine work

  ! macOS updates are available
    └─ Install available macOS updates
```

--verbose

```
PASSED

  ✓ Firewall is enabled
    Requirement: Firewall must be enabled
    Observed:    Enabled
    Evidence:    Application Firewall global state = enabled
```

## Testing

Unit tests
- [ ] Mock System

Integration tests
- [ ] Integration tests: disposable macOS VM
- [ ] Integration tests: Docker container
- [ ] Integration tests: Windows containerised VM
- [ ] CI integration test runners

## Checks

Bunch of potential checks

- [ ] ADD: https://github.com/ernw/hardening/blob/master/operating_system/osx/26/Hardening_Guide-macOS_26_Tahoe_1.0.md#ensure-system-volume-is-read-only
- [ ] ADD: `csrutil authenticated-root status` "Enable authenticated root"

Apple
- [ ] Require clamav to be running
- [ ] Require clamscan job to be running
- [ ] Require freshclam job to be running
- [ ] Require CPU 'no execute' to be enabled
- [ ] Require Secure Boot to be enabled
- [ ] Device Uptime: Require device to be restarted regularly
- [ ] Disk Health: Require sufficient space on primary disk
- [ ] Require screen lock configuration
- [ ] macOS sharing; bluetooth, remote apple events, app scripting, screen sharing
- [ ] Require encrypted storage of sensitive .env files
- [ ] sudo: Disallow passwordless invocation
- [ ] sudo: Require use_pty to be configured
- [ ] Require Find My Service to be enabled/disabled
- [ ] iTerm2: Require secure keyboard entry to be enabled
- [ ] Require Apple Intelligence to be disabled

Login and access:
- [ ] Ensure root account shells are set to nologin
- [ ] Ensure system account shells are set to nologin
- [ ] Require guest account shells to be disabled

Apps:
- [ ] Password manager is installed
- [ ] Require browser be up to date

Policy:
- [ ] Dropbox not installed
- [ ] App blocklist/allowlist

Dev:
- [ ] Require AWS credentials to be encrypted
- [ ] Block GitHub Copilot

Linux:
- [ ]
