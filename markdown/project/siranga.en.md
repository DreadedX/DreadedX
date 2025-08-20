---
project:
  url: git.huizinga.dev/Dreaded_X/siranga
  title: Siranga (Greek for Tunnel)
---

Sometimes you want to use your fancy new work-in-progress with someone remote, but how would you do this easily and securely?
That is where Siranga comes in, with this tool you can quickly create a new subdomain that connects to a local port on your machine, and all you need: SSH!
When connecting to Siranga over SSH it makes uses of the tunneling capabilities of SSH to forward one of your local ports to Siranga.
At the same time Siranga acts as a webserver and when a connection comes in for a given subdomain it will handle creating the connection through the SSH tunnel.

The authorized SSH keys for each user are retrieved through LDAP and the subdomains are (optionally) protected using ForwardAuth.
In my Kubernetes cluster LDAP is provided by LLDAP and ForwardAuth is provided by my single sign-on provider Authelia.
