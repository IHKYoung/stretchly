import 'chai/register-should'
import IdeasLoader from '../apps/desktop/legacy-utils/ideasLoader'
import Shuffled from '../apps/desktop/legacy-utils/shuffled'

describe('ideasLoader', function () {
  const ideas = new IdeasLoader([
    { data: 'a', enabled: true },
    { data: 'b', enabled: false },
    { data: 'c', enabled: true }
  ])

  it('returns enabled ideas', () => {
    const enabled = new Shuffled(['a', 'c'])
    ideas.ideas().should.be.deep.equal(enabled)
  })
})
