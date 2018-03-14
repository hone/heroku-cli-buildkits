import {Command, flags} from 'cli-engine-heroku'
import cli from 'heroku-cli-util'
import binary from 'node-pre-gyp'
import path from 'path'
import child from 'child_process'
var addonPath = binary.find(path.resolve(path.join(__dirname, '../../package.json')))
var addon = require(addonPath)

export default class Create extends Command {
  static topic = 'buildpacks'
  static command = 'register'
  static description = 'create a buildpack'
  static flags = {
    team: flags.team(),
    support: flags.string({description: 'method of support'})
  }
  static args = [
    {
      name: 'name',
      optional: true,
      description: 'name of the buildpack'
    },
    {
      name: 'url',
      optional: true,
      description: 'github repo URL for the buildpack, if not specified it will try to autodetect based off of the origin git remote'
    },
  ]

  async run () {
    let partner_portal = 'https://partner-portal-ui-staging.herokuapp.com/buildpacks/new'
    this.out.log("Opening the buildpack registration page...")
    cli.open(partner_portal)
    /*
    let support = this.flags.support || "";
    let nameParts = this.args.name.split('/')
    if (nameParts.length !== 2) {
      this.out.error(`Invalid buildpack name: ${this.args.name}`)
      return
    }
    let namespace = nameParts[0]
    let name = nameParts[1]

    if (this.args.url) {
      addon.register(this.args.url, namespace, name, this.flags.team, support)
    } else {
      child.exec('git remote get-url origin', (err, stdout, stderr) => {
        if (err) {
          this.out.error('Error getting repo URL')
        } else {
          if (stdout) {
            let repoUrl = `${stdout}`
            if (repoUrl.substring(0, 4) === 'http') {
              let repo = repoUrl.replace('.git', '')
              addon.register(repo, namespace, name, this.flags.team, support)
            } else if (repoUrl.substring(0, 14) === 'git@github.com') {
              let repo = repoUrl.replace('git@github.com:', 'https://github.com/').replace('.git', '')
              addon.register(repo, namespace, name, this.flags.team, support)
            } else {
              this.out.error(`Unrecognized repo URL: ${repoUrl}`)
            }
          } else {
            this.out.error("Git remote 'origin' not found. Must provide URL.")
          }
        }
      })
    }
    */
  }
}
