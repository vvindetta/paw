# PAW is modular multifactor module for PAM (wip)

## Install

1. Find where is paM modules dir in your system. Defoult is ```/lib64/security```

2. Install ```pam_paw.so``` in it

```sudo install -m 0644 -o root -g root pam_paw.so /usr/lib64/security/pam_paw.so```

2. Create ```paw``` directory

```sudo mkdir /lib64/security/paw```

3. Place paW modules in it. Example:
```/lib64/security/paw/paw_test.so```


## Testing

1. Install ```pamtester``` via your system package manager

2. Create and edit ```/etc/pam.d/paw_testing```

```
auth    required    pam_paw.so
account required    pam_permit.so
```

3. Run

```pamtester paw_testing $USER authenticate```