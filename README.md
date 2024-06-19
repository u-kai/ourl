# Short URL Opener

SURL is a CLI that opens a browser with a short URL.

## How to use

To open an oreil.ly short URL in the browser, use the command as shown below:

```bash
$ surl PATH_TO_SHORT_URL
```

To open other short URLs, use the command with the `-d` option for specifying the domain name:

```bash
$ surl -d SOMETHING_DOMAIN_NAME PATH_TO_SHORT_URL
```

The -d option allow you to specify the domain name when you want to open a different short URL.

If you want to set a default domain name, you can set the `DEFAULT_SURL_DOMAIN` environment variable:

```bash
$ export DEFAULT_SURL_DOMAIN=SOMETHING_DOMAIN_NAME
$ surl PATH_TO_SHORT_URL
```

## Example

Execute below command, then browser will open with https://www.redhat.com/en/topics/virtualization/what-is-a-virtual-machine

```bash
$ surl HEtBk
# redirect to https://www.redhat.com/en/topics/virtualization/what-is-a-virtual-machine
```

## Download

You can download the latest version of SURL from the [GitHub release page](https://github.com/u-kai/surl/releases)

## NOTE

### Windows Users

If you download the exe file for Windows, please be aware that running the command may be flagged as a virus by your antivirus software.

This is a common issue with executable files downloaded from the internet. If this happens, you may need to whitelist the executable in your antivirus settings.
