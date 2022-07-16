module.exports = function () {
  const sdkDouble = {
    login: function () {
      sdkDouble.login.called = true;
    },
  };
  return sdkDouble;
};
