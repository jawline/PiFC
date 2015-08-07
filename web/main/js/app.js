'use strict';

/* App Module */

var imcluelessSite = angular.module('imcluelessSite', [ 'ngRoute', 'RestServices']);

imcluelessSite.config([ '$routeProvider', function($routeProvider) {
	$routeProvider.when('/', {
		redirectTo : '/landing'
	}).when('/landing', {
		templateUrl : 'partials/landing.html',
		controller : 'LandingCtrl'
	}).when('/error404', {
		templateUrl : 'partials/error404.html'
	}).when('/logs', {
		templateUrl : 'partials/logs.html'
	}).when('/commands', {
		templateUrl : 'partials/commands.html',
		controller : 'CommandsCtrl'
	}).when('/status', {
		templateUrl: 'partials/status.html',
		controller: 'StatusCtrl'
	}).otherwise({
		redirectTo : '/error404'
	});
}]).run(function ($rootScope) {});
