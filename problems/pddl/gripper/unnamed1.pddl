﻿(define (problem strips-gripper-x-20)
   (:domain gripper-strips-shakey)
   (:objects room1 room2 room3 room4 inter1 inter2 inter3 inter4 ball4 ball3 ball2 ball1 left right)
   (:init (room room1)
          (room room2)
          (room room3)
          (room room4)
          (ball ball4)
          (ball ball3)
          (ball ball2)
          (ball ball1)
          (interrupt inter1)
          (interrupt inter2)
          (interrupt inter3)
          (interrupt inter4)
          (at-robby room3)
          (free left)
          (free right)
          (at ball4 room1)
          (at ball3 room1)
          (at ball2 room1)
          (at ball1 room1)
          (gripper left)
          (gripper right)
          (at-i inter1 room1)
          (at-i inter2 room2)
          (at-i inter3 room3)
          (at-i inter4 room4))
   (:goal   (and (at ball2 room2)
            (allum inter4)
            )
               ))