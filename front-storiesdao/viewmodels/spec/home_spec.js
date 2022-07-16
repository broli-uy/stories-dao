describe('createHomeViewmodel', function () {
  var createHomeViewmodel = require('../home');
  var sdkDouble = require('./support/doubles/sdk');

  it('starts web3auth', function () {
    var sdk = sdkDouble();
    var vm = createHomeViewmodel(sdk);
    expect(sdk.login.called).toBeTrue();
  });
});
