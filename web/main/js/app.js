'use strict';

/* App Module */

var imcluelessSite = angular.module('imcluelessSite', [ 'ngRoute', 'RestServices']);

imcluelessSite.config([ '$routeProvider', function($routeProvider) {
	$routeProvider.when('/', {
		redirectTo : '/landing'
	}).when('/landing', {
		templateUrl : 'partials/landing.html',
		controller : 'LandingCtrl',
		redirectTo : '/status'
	}).when('/error404', {
		templateUrl : 'partials/error404.html'
	}).when('/logs', {
		templateUrl : 'partials/logs.html',
		controller: 'LogsCtrl'
	}).when('/commands', {
		templateUrl : 'partials/commands.html',
		controller : 'CommandsCtrl'
	}).when('/status', {
		templateUrl: 'partials/status.html',
		controller: 'StatusCtrl'
	}).when('/configs', {
		templateUrl: 'partials/configs.html',
		controller: 'ConfigsCtrl'
	}).otherwise({
		redirectTo : '/error404'
	});
}]).run(function ($rootScope, $location) {
	$rootScope.$watch(function() {
		return $location.path();
   	}, function(newValue, oldValue) {}, true);
});
