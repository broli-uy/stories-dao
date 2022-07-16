module.exports = function createHomeViewmodel(sdk) {
  const vm = {};

  sdk.login();

  return vm;
};
