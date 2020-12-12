using System;
using System.Collections.Generic;
using System.Linq;

namespace _12
{
    public class Ship
    {
        private (int x, int y) _startingPoint;
        private (int x, int y) _location;
        private readonly Queue<(char, int)> _instructions;
        private bool _useWaypoint;
        private (int x, int y) _waypoint;
        private (int x, int y) _waypointLocation;
        
        private int _direction;
        private readonly (int, char)[] _directions = { (0, 'N'), (90, 'E'), (180, 'S'), (270, 'W')};
        
        public Ship(string[] instructions, char initialDirection = 'E')
        {
            _instructions = new Queue<(char, int)>();
            _startingPoint = (0, 0);
            _location = (0, 0);
            _direction = GetDirection(initialDirection);
            foreach (var s in instructions)
                _instructions.Enqueue((s[0], Convert.ToInt32(s[1..])));
        }

        public void UseWaypoint(int x = 10, int y = -1)
        {
            _waypoint = (x, y);
            _waypointLocation = _waypoint;
            _useWaypoint = true;
        }
        
        private int GetDirection(char d) => _directions.First(x => x.Item2 == d).Item1;
        private char GetDirection(int d) => _directions.First(x => x.Item1 == d).Item2;

        private int FloorDegrees(int deg)
        {
            if (deg >= 360) return deg - 360;
            if (deg < 0) return deg + 360;
            return deg;
        }
        private void SetDirection(char dir, int val)
        {
            int next = FloorDegrees(_direction + (dir == 'L' ? val * -1 : val));

            if (_useWaypoint)
            {
                double angle = FloorDegrees(360 - _direction + next);
                angle = (Math.PI / 180) * angle;
                
                double sin = Math.Round(Math.Sin(angle));
                double cos = Math.Round(Math.Cos(angle));
                double x = _waypoint.x * cos - _waypoint.y * sin;
                double y = _waypoint.x * sin + _waypoint.y * cos;
                _waypoint = ((int)x, (int)y);
                _waypointLocation = _location;
                _waypointLocation.x += _waypoint.x;
                _waypointLocation.y += _waypoint.y;
            }
            else
                _direction = next;
        }

        private void Move(char dir, int val)
        {
            if(!_useWaypoint)
                dir = dir == 'F' ? GetDirection(_direction) : dir;
            
            var movement = dir switch
            {
                'N' => (-val, 0),
                'E' => (0, val),
                'S' => (val, 0),
                'W' => (0, -val),
                _ => (0,0)
            };

            if (_useWaypoint)
            {
                if (dir == 'F')
                {
                    _location.x += _waypoint.x * val;
                    _location.y += _waypoint.y * val;
                    _waypointLocation.x += _waypoint.x * val;
                    _waypointLocation.y += _waypoint.y * val;
                }
                else
                {
                    _waypointLocation.x += movement.Item2;
                    _waypointLocation.y += movement.Item1;
                    _waypoint.x += movement.Item2;
                    _waypoint.y += movement.Item1;
                }
            }
            else
            {
                _location.x += movement.Item2;
                _location.y += movement.Item1;
            }
        }

        public int Distance()
            => Math.Abs(_location.x - _startingPoint.x) + Math.Abs(_location.y - _startingPoint.y);

        public void SetSail()
        {
            while (_instructions.Any())
            {
                var i = _instructions.Dequeue();

                if (i.Item1 == 'L' || i.Item1 == 'R')
                    SetDirection(i.Item1, i.Item2);
                else
                    Move(i.Item1, i.Item2);
            }
        }
        
    }
}