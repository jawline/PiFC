'use strict';

var API_PORT = "14320";
var API_URL = location.protocol+'//'+location.hostname + ":" + API_PORT;

/* Controllers */
function LandingCtrl($scope, $http) {}

function CommandsCtrl($scope, $http) {
	$scope.api_url = API_URL;
}