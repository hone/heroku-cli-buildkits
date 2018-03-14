export default function push (remote) {
  return `git push ${remote || 'heroku'} master`
}
