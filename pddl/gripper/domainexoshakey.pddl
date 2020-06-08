(define (domain gripper-strips-shakey)
   (:predicates (room ?r)
		(ball ?b)
		(gripper ?g)
		(interrupt ?i)
		(at-robby ?r)
		(at ?b ?r)
		(at-i ?i ?r)
		(allum ?i)
		(free ?g)
		(carry ?o ?g))

   (:action move
       :parameters  (?from ?to)
       :precondition (and  (room ?from) (room ?to) (at-robby ?from))
       :effect (and  (at-robby ?to)
		     (not (at-robby ?from))))



   (:action pick
       :parameters (?obj ?room ?gripper)
       :precondition  (and  (ball ?obj) (room ?room) (gripper ?gripper)
			    (at ?obj ?room) (at-robby ?room) (free ?gripper))
       :effect (and (carry ?obj ?gripper)
		    (not (at ?obj ?room)) 
		    (not (free ?gripper))))


   (:action drop
       :parameters  (?obj  ?room ?gripper)
       :precondition  (and  (ball ?obj) (room ?room) (gripper ?gripper)
			    (carry ?obj ?gripper) (at-robby ?room))
       :effect (and (at ?obj ?room)
		    (free ?gripper)
		    (not (carry ?obj ?gripper))))
		    
    (:action on
       :parameters  (?interrupt  ?room ?gripper)
       :precondition  (and  (interrupt ?interrupt) (room ?room) (gripper ?gripper)
			    (at-i ?interrupt ?room) (at-robby ?room))
       :effect 
		    (allum ?interrupt)
		    )

    (:action off
       :parameters  (?interrupt  ?room ?gripper)
       :precondition  (and  (interrupt ?interrupt) (room ?room) (gripper ?gripper)
			    (at-i ?interrupt ?room) (at-robby ?room))
       :effect 
		    (not(allum ?interrupt))
		    )
		    

		    
)

